#[cfg(test)]
pub mod overlay {
    use std::path::PathBuf;
    use i_shape::int::path::IntPath;
    use i_shape::int::shape::IntShape;
    use serde::{Deserialize, Deserializer};
    use i_overlay::core::fill_rule::FillRule;

    #[derive(Debug, Deserialize)]
    pub struct Test {

        #[serde(rename = "fillRule")]
        #[serde(default, deserialize_with = "deserialize_fill_rule")]
        pub fill_rule: Option<FillRule>,
        #[serde(rename = "subjPaths")]
        pub subj_paths: Vec<IntPath>,
        #[serde(rename = "clipPaths")]
        pub clip_paths: Vec<IntPath>,
        pub clip: Vec<Vec<IntShape>>,
        pub subject: Vec<Vec<IntShape>>,
        pub difference: Vec<Vec<IntShape>>,
        #[serde(rename = "inverseDifference")]
        pub inverse_difference: Vec<Vec<IntShape>>,
        pub intersect: Vec<Vec<IntShape>>,
        pub union: Vec<Vec<IntShape>>,
        pub xor: Vec<Vec<IntShape>>,
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