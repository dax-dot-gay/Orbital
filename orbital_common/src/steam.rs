use std::path::{Path, PathBuf};

pub struct SteamLibrary(PathBuf);

impl SteamLibrary {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self(path.as_ref().to_path_buf())
    }

    pub fn base(&self) -> PathBuf {
        self.0.clone()
    }

    pub fn game_path(&self) -> PathBuf {
        self.0.join("steamapps/common/Satisfactory")
    }

    pub fn community_resources(&self) -> PathBuf {
        self.game_path().join("CommunityResources")
    }

    pub fn docs(&self) -> PathBuf {
        self.community_resources().join("Docs")
    }

    pub fn paks(&self) -> PathBuf {
        self.game_path().join("FactoryGame/Content/Paks")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn game_path() {
        let config = TestConfiguration::new();
        let library = SteamLibrary::new(config.steam.steam_library.as_path());
        assert_eq!(library.game_path(), config.steam.expect.game_folder, "GAME_PATH: {:?} != {:?}", library.game_path(), config.steam.expect.game_folder);
    }

    #[test]
    fn community_resources() {
        let config = TestConfiguration::new();
        let library = SteamLibrary::new(config.steam.steam_library.as_path());
        assert_eq!(library.community_resources(), config.steam.expect.community_folder, "COMMUNITY_RESOURCES: {:?} != {:?}", library.community_resources(), config.steam.expect.community_folder);
    }

    #[test]
    fn docs() {
        let config = TestConfiguration::new();
        let library = SteamLibrary::new(config.steam.steam_library.as_path());
        assert_eq!(library.docs(), config.steam.expect.docs_folder, "DOCS: {:?} != {:?}", library.docs(), config.steam.expect.docs_folder);
    }

    #[test]
    fn paks() {
        let config = TestConfiguration::new();
        let library = SteamLibrary::new(config.steam.steam_library.as_path());
        assert_eq!(library.paks(), config.steam.expect.paks_folder, "PAKS: {:?} != {:?}", library.paks(), config.steam.expect.paks_folder);
    }
}
