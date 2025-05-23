use duct::cmd;
use walkdir::WalkDir;
use zip::{write::SimpleFileOptions, ZipWriter};
use std::{
    collections::HashMap, env::set_current_dir, error::Error, fs, io::{BufRead, BufReader, Read, Write}, os::unix::fs::PermissionsExt, path::{Path, PathBuf}
};

use clap::Parser;
use cli::Cli;
use orbital_common::{
    steam::SteamLibrary,
    types::satisfactory::{AssetReference, Generated, Generator, parse_docs_json},
};
use serde_json::to_string_pretty;
use tempfile::tempdir;

mod cli;

include!(concat!(env!("OUT_DIR"), "/binaries.rs"));

fn generate_asset_request(generated: Generated, workdir: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut requests: HashMap<String, String> = HashMap::new();
    for (_, desc) in generated.descriptions {
        for item in [desc.big_icon, desc.icon] {
            if let Some(AssetReference {
                asset_type,
                asset_path,
                asset_id: Some(id),
            }) = item
            {
                if !requests.contains_key(&id) && asset_type == String::from("Texture2D") {
                    requests.insert(
                        id,
                        format!(
                            "TEXTURE::/FactoryGame/Content/{}",
                            asset_path
                                .trim_start_matches('/')
                                .split_once('/')
                                .unwrap()
                                .1
                        ),
                    );
                }
            }
        }
    }

    requests.insert(String::from("MapSlice0_0"), String::from("TEXTURE::/FactoryGame/Content/FactoryGame/Interface/UI/Assets/MapTest/SlicedMap/Map_0-0.Map_0-0"));
    requests.insert(String::from("MapSlice1_0"), String::from("TEXTURE::/FactoryGame/Content/FactoryGame/Interface/UI/Assets/MapTest/SlicedMap/Map_1-0.Map_1-0"));
    requests.insert(String::from("MapSlice0_1"), String::from("TEXTURE::/FactoryGame/Content/FactoryGame/Interface/UI/Assets/MapTest/SlicedMap/Map_0-1.Map_0-1"));
    requests.insert(String::from("MapSlice1_1"), String::from("TEXTURE::/FactoryGame/Content/FactoryGame/Interface/UI/Assets/MapTest/SlicedMap/Map_1-1.Map_1-1"));

    let mut file = fs::File::create(workdir.join("asset_req.txt"))?;
    for (id, tail) in requests {
        file.write(format!("{id}::{tail}\n").as_bytes())?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = Cli::parse();
    let (workdir, _tmp_workdir) = if let Some(workpath) = options.workdir {
        if !workpath.exists() {
            fs::create_dir_all(workpath.clone())
                .expect("Working directory does not exist, and could not be created.");
        }
        (workpath, None)
    } else {
        let tmp = tempdir().expect("Unable to create tempdir for working directory.");
        (tmp.path().to_path_buf(), Some(tmp))
    };

    let output_path = options
        .output
        .unwrap_or(Path::new("./extracted").to_path_buf());
    if !output_path.exists() {
        fs::create_dir_all(output_path.clone()).expect("Failed to create output directory");
    }

    let steam = SteamLibrary::new(options.steam_library.as_path());
    let locale = options.locale.unwrap_or(String::from("en-US"));
    let parsed = parse_docs_json(steam.docs(), locale)?;
    let generated = Generator::new(parsed).generate();

    fs::write(workdir.join("docs.json"), to_string_pretty(&generated)?)?;
    generate_asset_request(generated.clone(), workdir.clone())?;

    let ((exe_filename, exe_content), (lib_filename, lib_content)) = binaries();
    fs::write(workdir.join(exe_filename.clone()), exe_content)?;
    fs::write(workdir.join(lib_filename.clone()), lib_content)?;

    #[cfg(unix)]
    {
        let mut exe_perms = fs::metadata(workdir.join(exe_filename.clone()))?.permissions();
        exe_perms.set_mode(0o777);
        fs::set_permissions(workdir.join(exe_filename.clone()), exe_perms)?;

        let mut lib_perms = fs::metadata(workdir.join(lib_filename.clone()))?.permissions();
        lib_perms.set_mode(0o777);
        fs::set_permissions(workdir.join(lib_filename.clone()), lib_perms)?;
    }

    let _paks = steam.paks();
    let _comr = steam.community_resources();

    let sc_args = vec![
        _paks.to_str().unwrap(),
        _comr.to_str().unwrap(),
        "asset_req.txt",
        lib_filename.as_str(),
    ];
    let sidecar = cmd(exe_filename, &sc_args);
    let reader = sidecar.dir(workdir.clone()).stderr_to_stdout().reader()?;
    let lines = BufReader::new(reader).lines();

    for line in lines {
        match line {
            Ok(val) => {
                println!("SIDECAR: {}", val);
            }
            Err(e) => {
                println!("SIDECAR FAILED: {e:?}");
                break;
            }
        };
    }

    fs::create_dir_all(workdir.join("staging").join("map"))?;
    fs::create_dir_all(workdir.join("staging").join("icons"))?;
    fs::rename(workdir.join("docs.json"), workdir.join("staging").join("docs.json"))?;

    for fp in glob::glob(workdir.join("assets").join("*.png").to_str().unwrap())? {
        if let Ok(pt) = fp {
            if let Some(fname) = pt.file_name() {
                if fname.to_str().unwrap().starts_with("MapSlice") {
                    fs::rename(pt.clone(), workdir.join("staging").join("map").join(fname))?;
                } else {
                    fs::rename(pt.clone(), workdir.join("staging").join("icons").join(fname))?;
                }
            }
        }
    }

    let mut output_file = fs::File::create(output_path.clone().join("assets.zip"))?;
    let mut zipfile = ZipWriter::new(&mut output_file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let mut buffer: Vec<u8> = Vec::new();
    set_current_dir(workdir.join("staging"))?;
    for file_path in WalkDir::new(".") {
        if let Ok(path) = file_path.and_then(|p| Ok(p.path().to_path_buf())) {
            if path.is_file() {
                zipfile.start_file_from_path(&path, options.clone())?;

                let mut f = fs::File::open(path)?;
                f.read_to_end(&mut buffer)?;
                zipfile.write_all(&*buffer)?;
                buffer.clear();
            } else if path.as_os_str().len() != 0 {
                zipfile.add_directory_from_path(&path, options.clone())?;
            }
        }
    }

    Ok(())
}
