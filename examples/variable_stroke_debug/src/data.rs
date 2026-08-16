use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct FixtureVertex {
    pub(crate) point: [f32; 2],
    pub(crate) width: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Fixture {
    pub(crate) scale: f32,
    pub(crate) stroke: Vec<Vec<FixtureVertex>>,
}

pub(crate) struct FixtureResource {
    files: Vec<PathBuf>,
}

impl FixtureResource {
    pub(crate) fn new(folder: impl AsRef<Path>) -> Self {
        let mut files: Vec<_> = std::fs::read_dir(folder)
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
            .collect();
        files.sort_by_key(|path| fixture_index(path).unwrap_or(usize::MAX));
        Self { files }
    }

    pub(crate) fn len(&self) -> usize {
        self.files.len()
    }

    pub(crate) fn name(&self, index: usize) -> String {
        self.files
            .get(index)
            .and_then(|path| path.file_stem())
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_owned()
    }

    pub(crate) fn load(&self, index: usize) -> Result<Fixture, String> {
        let path = self
            .files
            .get(index)
            .ok_or_else(|| format!("fixture index {index} is out of range"))?;
        let content = std::fs::read_to_string(path)
            .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
        serde_json::from_str(&content)
            .map_err(|error| format!("failed to parse {}: {error}", path.display()))
    }
}

fn fixture_index(path: &Path) -> Option<usize> {
    path.file_stem()?
        .to_str()?
        .strip_prefix("test_")?
        .parse()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::FixtureResource;
    use std::path::PathBuf;

    #[test]
    fn discovers_repro_fixtures_in_numeric_order() {
        let folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../tests/variable_stroke");
        let resource = FixtureResource::new(folder);

        assert!(resource.len() >= 13);
        for index in [10, 11, 12] {
            assert_eq!(resource.name(index), format!("test_{index}"));
            assert!(!resource.load(index).unwrap().stroke.is_empty());
        }
    }
}
