use std::collections::HashMap;
use std::path::PathBuf;
use i_triangle::i_overlay::i_shape::int::path::IntPath;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct PolygonTest {
    #[serde(rename = "subjPaths")]
    pub(crate) subj_paths: Vec<IntPath>,
    #[serde(rename = "clipPaths")]
    pub(crate) clip_paths: Vec<IntPath>,
}

impl PolygonTest {
    fn load(index: usize, folder: &str) -> Option<Self> {
        let file_name = format!("test_{}.json", index);
        let mut path_buf = PathBuf::from(folder);
        path_buf.push(file_name);

        let data = match std::fs::read_to_string(path_buf.as_path()) {
            Ok(data) => {
                data
            }
            Err(e) => {
                eprintln!("{:?}", e);
                return None;
            }
        };

        let result: Result<PolygonTest, _> = serde_json::from_str(&data);
        match result {
            Ok(test) => Some(test),
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                return None;
            }
        }
    }

    fn tests_count(folder: &str) -> usize {
        let folder_path = PathBuf::from(folder);
        match std::fs::read_dir(folder_path) {
            Ok(entries) => {
                entries
                    .filter_map(|entry| {
                        entry.ok().and_then(|e| {
                            let path = e.path();
                            if path.extension()?.to_str()? == "json" {
                                Some(())
                            } else {
                                None
                            }
                        })
                    })
                    .count()
            }
            Err(e) => {
                eprintln!("Failed to read directory: {}", e);
                0
            }
        }
    }
}

pub(crate) struct PolygonResource {
    folder: String,
    pub(crate) count: usize,
    pub(crate) tests: HashMap<usize, PolygonTest>
}

impl PolygonResource {
    pub(crate) fn new(folder: &str) -> Self {
        let count = PolygonTest::tests_count(folder);
        Self { count, folder: folder.to_string(), tests: Default::default() }
    }

    pub(crate) fn load(&mut self, index: usize) -> Option<PolygonTest> {
        if self.count <= index {
            return None;
        }
        if let Some(test) = self.tests.get(&index) {
            return Some(test.clone())
        }

        let test = PolygonTest::load(index, self.folder.as_str())?;

        self.tests.insert(index, test.clone());

        Some(test)
    }
}
