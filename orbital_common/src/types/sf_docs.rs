use std::{char::decode_utf16, fs::File, io::Read, path::PathBuf};

use serde_json::Value;
use strip_bom::StripBom;

use crate::DocsError;

pub(crate) fn parse_docs_json(docs_folder: PathBuf, locale: String) -> crate::Result<Value> {
    let combined_path = docs_folder.join(locale.clone() + ".json");
    if !combined_path.exists() {
        return Err(DocsError::unknown_locale(locale).into());
    }

    let mut file_bytes: Vec<u8> = Vec::new();
    let file_bytesize = File::open(&combined_path)
        .or_else(|e| {
            Err::<File, crate::Error>(DocsError::failed_read(combined_path.as_path(), e).into())
        })?
        .read_to_end(&mut file_bytes)
        .or_else(|e| {
            Err::<usize, crate::Error>(DocsError::failed_read(combined_path.as_path(), e).into())
        })?;

    if file_bytesize % 2 != 0 {
        return Err(
            DocsError::invalid_format("File is not UTF-16 encoded/is missing bytes.").into(),
        );
    }
    let iter = (0..(file_bytesize / 2))
        .map(|i| u16::from_le_bytes([file_bytes[2 * i], file_bytes[2 * i + 1]]));

    let decoded = decode_utf16(iter)
        .collect::<Result<String, _>>()
        .or_else(|e| {
            Err::<String, crate::Error>(
                DocsError::invalid_format(format!("Bad UTF-16 encoding: {:?}", e)).into(),
            )
        })?;

    serde_json::from_str::<Value>(&decoded.strip_bom())
        .or_else(|e| Err(DocsError::invalid_format(format!("Bad JSON data: {:?}", e)).into()))
}

#[cfg(test)]
mod tests {
    use crate::{steam::SteamLibrary, test_utils::TestConfiguration};

    use super::*;

    #[test]
    fn test_docs_initial_parse() -> crate::Result<()> {
        let config = TestConfiguration::new();
        let library = SteamLibrary::new(config.steam.steam_library.as_path());
        let result = parse_docs_json(library.docs(), config.docs.locale)?;
        assert!(result.is_array(), "Expected array");
        Ok(())
    }
}
