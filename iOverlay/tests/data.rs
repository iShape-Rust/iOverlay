#[cfg(test)]
pub mod overlay {
    // extern crate std;

    use std::path::PathBuf;
    use i_shape::int::path::IntPaths;
    use i_shape::int::shape::{IntContour, IntShapes};
    use serde::{Deserialize, Deserializer};
    use i_overlay::core::fill_rule::FillRule;

    fn deserialize_fill_rule<'de, D>(deserializer: D) -> Result<Option<FillRule>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = Option::<i32>::deserialize(deserializer)?;
        match val {
            Some(0) => Ok(Some(FillRule::EvenOdd)),
            Some(1) => Ok(Some(FillRule::NonZero)),
            None => Ok(None), // This covers the case where the field is missing
            _ => Err(serde::de::Error::custom("Invalid value for FillRule")),
        }
    }

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    pub struct BooleanTest {
        #[serde(rename = "fillRule")]
        #[serde(default, deserialize_with = "deserialize_fill_rule")]
        pub fill_rule: Option<FillRule>,
        #[serde(rename = "subjPaths")]
        pub subj_paths: Vec<IntContour>,
        #[serde(rename = "clipPaths")]
        pub clip_paths: Vec<IntContour>,
        pub clip: Vec<IntShapes>,
        pub subject: Vec<IntShapes>,
        pub difference: Vec<IntShapes>,
        #[serde(rename = "inverseDifference")]
        pub inverse_difference: Vec<IntShapes>,
        pub intersect: Vec<IntShapes>,
        pub union: Vec<IntShapes>,
        pub xor: Vec<IntShapes>,
    }

    impl BooleanTest {
        #[allow(dead_code)]
        pub fn load(index: usize) -> Self {
            let file_name = format!("test_{}.json", index);
            let mut path_buf = PathBuf::from("./tests/boolean");
            path_buf.push(file_name);

            let data = match std::fs::read_to_string(path_buf.as_path()) {
                Ok(data) => {
                    data
                }
                Err(e) => {
                    panic!("{:?}", e);
                }
            };

            let result: Result<BooleanTest, _> = serde_json::from_str(&data);
            match result {
                Ok(test) => test,
                Err(e) => {
                    eprintln!("Failed to parse JSON: {}", e);
                    panic!("can not parse file");
                }
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    pub struct StringTest {
        #[serde(rename = "fillRule")]
        #[serde(default, deserialize_with = "deserialize_fill_rule")]
        pub fill_rule: Option<FillRule>,
        pub body: Vec<IntContour>,
        pub string: IntPaths,
        pub slice: Vec<IntShapes>,
        pub clip_direct: Vec<IntPaths>,
        pub clip_invert: Vec<IntPaths>,
    }

    impl StringTest {
        #[allow(dead_code)]
        pub fn load(index: usize) -> Self {
            let file_name = format!("test_{}.json", index);
            let mut path_buf = PathBuf::from("./tests/string");
            path_buf.push(file_name);

            let data = match std::fs::read_to_string(path_buf.as_path()) {
                Ok(data) => {
                    data
                }
                Err(e) => {
                    panic!("{:?}", e);
                }
            };

            let result: Result<StringTest, _> = serde_json::from_str(&data);
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
