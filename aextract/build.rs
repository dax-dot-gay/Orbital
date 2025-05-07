use std::{path::Path, process::Command};

fn main() {
    println!("cargo::rerun-if-changed=AextractSidecar/Program.cs");
    println!("cargo::rerun-if-changed=AextractSidecar/AextractSidecar.csproj");
    println!("cargo::rerun-if-changed=build.rs");
    let publish_linux = Command::new("dotnet")
        .arg("publish")
        .arg("-r")
        .arg("linux-x64")
        .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("AextractSidecar"))
        .spawn()
        .expect("Should be able to compile sidecar for linux.");
    let publish_windows = Command::new("dotnet")
        .arg("publish")
        .arg("-r")
        .arg("win-x64")
        .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("AextractSidecar"))
        .spawn()
        .expect("Should be able to compile sidecar for windows.");

    let linout = publish_linux
        .wait_with_output()
        .expect("Should be able to complete linux publish");
    let winout = publish_windows
        .wait_with_output()
        .expect("Should be able to complete windows publish");
    println!("LINUX.OUT: {}", String::from_utf8(linout.stdout).unwrap());
    println!("LINUX.ERR: {}", String::from_utf8(linout.stderr).unwrap());
    println!("WINDOWS.OUT: {}", String::from_utf8(winout.stdout).unwrap());
    println!("WINDOWS.ERR: {}", String::from_utf8(winout.stderr).unwrap());
}
