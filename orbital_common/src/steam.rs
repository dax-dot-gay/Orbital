use std::path::{Path, PathBuf};

pub struct SteamLibrary(PathBuf);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn getenv() {
        println!("{:?}", std::env::vars());
    }
}
