use std::{
    collections::HashMap,
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
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
                    requests.insert(id, format!("TEXTURE::{asset_path}"));
                }
            }
        }
    }

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
        fs::create_dir_all(output_path).expect("Failed to create output directory");
    }

    let steam = SteamLibrary::new(options.steam_library.as_path());
    let locale = options.locale.unwrap_or(String::from("en-US"));
    let parsed = parse_docs_json(steam.docs(), locale)?;
    let generated = Generator::new(parsed).generate();

    fs::write(workdir.join("docs.json"), to_string_pretty(&generated)?)?;
    generate_asset_request(generated.clone(), workdir)?;

    Ok(())
}
