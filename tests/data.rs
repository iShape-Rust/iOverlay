pub mod overlay {
    use std::path::PathBuf;
    use i_shape::fix_path::FixPath;
    use i_shape::fix_shape::FixShape;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Test {
        #[serde(rename = "subjPaths")]
        pub subj_paths: Vec<FixPath>,
        #[serde(rename = "clipPaths")]
        pub clip_paths: Vec<FixPath>,
        pub clip: Vec<FixShape>,
        pub subject: Vec<FixShape>,
        pub difference: Vec<FixShape>,
        pub intersect: Vec<FixShape>,
        pub union: Vec<FixShape>,
        pub xor: Vec<FixShape>
    }

    impl Test {
        pub fn load(index: usize) -> Self {
            let file_name = format!("test_{}.json", index);
            let mut path_buf = PathBuf::from("./tests/data");
            path_buf.push(file_name);

            let data = match std::fs::read_to_string(path_buf.as_path()) {
                Ok(data) => {
                    data
                },
                Err(e) => {
                    panic!("{:?}", e);
                }
            };

            let result: Result<Test, _> = serde_json::from_str(&data);
            match result {
                Ok(test) => test,
                Err(e) => {
                    eprintln!("Failed to parse JSON: {}", e);
                    panic!("can not parse file");
                }
            }
        }
    }
}