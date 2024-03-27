pub mod overlay {
    use std::path::PathBuf;
    use i_shape::fix_path::FixPath;
    use i_shape::fix_shape::FixShape;
    use serde::{Deserialize, Deserializer};
    use i_overlay::bool::fill_rule::FillRule;

    #[derive(Debug, Deserialize)]
    pub struct Test {

        #[serde(rename = "fillRule")]
        #[serde(default, deserialize_with = "deserialize_fill_rule")]
        pub fill_rule: Option<FillRule>,
        #[serde(rename = "subjPaths")]
        pub subj_paths: Vec<FixPath>,
        #[serde(rename = "clipPaths")]
        pub clip_paths: Vec<FixPath>,
        pub clip: Vec<Vec<FixShape>>,
        pub subject: Vec<Vec<FixShape>>,
        pub difference: Vec<Vec<FixShape>>,
        pub intersect: Vec<Vec<FixShape>>,
        pub union: Vec<Vec<FixShape>>,
        pub xor: Vec<Vec<FixShape>>,
    }

    fn deserialize_fill_rule<'de, D>(deserializer: D) -> Result<Option<FillRule>, D::Error>
        where D: Deserializer<'de>
    {
        let val = Option::<i32>::deserialize(deserializer)?;
        match val {
            Some(0) => Ok(Some(FillRule::EvenOdd)),
            Some(1) => Ok(Some(FillRule::NonZero)),
            None => Ok(None), // This covers the case where the field is missing
            _ => Err(serde::de::Error::custom("Invalid value for FillRule")),
        }
    }

    impl Test {
        pub fn load(index: usize) -> Self {
            let file_name = format!("test_{}.json", index);
            let mut path_buf = PathBuf::from("./tests/data");
            path_buf.push(file_name);

            let data = match std::fs::read_to_string(path_buf.as_path()) {
                Ok(data) => {
                    data
                }
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