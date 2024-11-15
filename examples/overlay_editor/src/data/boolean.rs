use std::collections::HashMap;
use std::path::PathBuf;
use i_triangle::i_overlay::i_shape::int::path::IntPath;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct BooleanTest {
    #[serde(rename = "subjPaths")]
    pub(crate) subj_paths: Vec<IntPath>,
    #[serde(rename = "clipPaths")]
    pub(crate) clip_paths: Vec<IntPath>,
}

impl BooleanTest {
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

        let result: Result<BooleanTest, _> = serde_json::from_str(&data);
        match result {
            Ok(test) => Some(test),
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                None
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

pub(crate) struct BooleanResource {
    folder: Option<String>,
    pub(crate) count: usize,
    pub(crate) tests: HashMap<usize, BooleanTest>
}

impl BooleanResource {
    pub(crate) fn with_path(folder: &str) -> Self {
        let count = BooleanTest::tests_count(folder);
        Self { count, folder: Some(folder.to_string()), tests: Default::default() }
    }

    pub(crate) fn with_content(content: String) -> Self {
        let tests_vec: Vec<BooleanTest> = serde_json::from_str(&content).unwrap_or_else(|e| {
            eprintln!("Failed to parse JSON content: {}", e);
            vec![]
        });

        let tests: HashMap<usize, BooleanTest> = tests_vec
            .into_iter()
            .enumerate() // Assign indices
            .collect();

        let count = tests.len();
        Self {
            count,
            folder: None,
            tests,
        }
    }

    pub(crate) fn load(&mut self, index: usize) -> Option<BooleanTest> {
        if self.count <= index {
            return None;
        }
        if let Some(test) = self.tests.get(&index) {
            return Some(test.clone())
        }

        let folder = if let Some(folder) = &self.folder { folder } else { return None; };
        let test = BooleanTest::load(index, folder.as_str())?;

        self.tests.insert(index, test.clone());

        Some(test)
    }
}
