# Orbital
Attempt 2 at a satisfactory planning tool

## Asset Extraction

Assets can be extracted from your local Satisfactory install into a zipped archive (containing only assets & information related to Orbital) using `aextract`. In addition to rust dependencies, this script requires additional setup, as follows:

### AExtract Requirements

- lib-skiasharp system library (for C# asset extraction). Installation varies by platform.
- .NET runtime

The extraction process also relies on the Oodle decompression library, which will be automatically downloaded for the current OS. The C# sidecar doesn't need to be manually compiled; it will be automatically built and packed into the Rust executable at build time.

### Usage

After installing its dependencies, enter the aextract directory and run the following:

```bash
# STEAMDIR is the base Steam Library folder that Satisfactory is in. On linux, this is the folder that contains the `steamapps` directory.
cargo run -- <STEAMDIR>
```

By default, an `assets.zip` file will be output to the `extracted` folder, which will be created if it doesn't exist. This can then be used as an asset pack for Orbital.
