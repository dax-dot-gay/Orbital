mod building;
mod description;
mod generator;
mod recipe;
mod research;
mod uestring;
mod utility;

use std::collections::HashMap;

pub use building::{BuildingFuelType, BuildingItem};
pub use description::{
    DescriptionEquipmentSlot, DescriptionGasType, DescriptionItem, DescriptionStackSize,
    DescriptionType,
};
pub use generator::{Generated, Generator};
pub use recipe::RecipeItem;
pub use research::{ResearchItem, ResearchType};
use serde::{Deserialize, Serialize};
use specta::Type;
pub use utility::{ClassReference, Coercion, IconPath, NormalizedString, AssetReference, parse_docs_json};

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct OrbitalData {
    pub research: HashMap<String, ResearchItem>,
    pub descriptions: HashMap<String, DescriptionItem>,
    pub buildables: HashMap<String, BuildingItem>,
    pub recipes: HashMap<String, RecipeItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(tag = "item_type")]
#[serde(rename_all = "snake_case")]
pub enum OrbitalItem {
    Research(ResearchItem),
    Description(DescriptionItem),
    Buildable(BuildingItem),
    Recipe(RecipeItem),
}

impl OrbitalData {
    pub fn get_id(&self, id: String) -> Option<OrbitalItem> {
        if let Some(item) = self.research.get(&id) {
            return Some(OrbitalItem::Research(item.clone()));
        }

        if let Some(item) = self.descriptions.get(&id) {
            return Some(OrbitalItem::Description(item.clone()));
        }

        if let Some(item) = self.buildables.get(&id) {
            return Some(OrbitalItem::Buildable(item.clone()));
        }

        if let Some(item) = self.recipes.get(&id) {
            return Some(OrbitalItem::Recipe(item.clone()));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Write, path::Path};

    use serde_json::to_string_pretty;

    use crate::{steam::SteamLibrary, test_utils::TestConfiguration};

    use super::*;

    #[test]
    fn test_docs_initial_parse() -> crate::Result<()> {
        let config = TestConfiguration::new();
        let library = SteamLibrary::new(config.steam.steam_library.as_path());
        let result = utility::parse_docs_json(library.docs(), config.docs.locale)?;
        assert!(result.is_array(), "Expected array");
        Ok(())
    }

    #[test]
    fn test_docs_normalize() -> crate::Result<()> {
        let config = TestConfiguration::new();
        let library = SteamLibrary::new(config.steam.steam_library.as_path());
        let result = utility::parse_docs_json(library.docs(), config.docs.locale)?;
        let mut generator = Generator::new(result);
        let generated = generator.generate();
        let mut f = fs::File::create(Path::new(env!("CARGO_MANIFEST_DIR")).join(config.docs.output)).unwrap();
        f.write_all(to_string_pretty(&generated).unwrap().as_bytes())
            .unwrap();

        Ok(())
    }
}
