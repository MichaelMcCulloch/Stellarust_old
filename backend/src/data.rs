use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MyJsonFile {
    pub name: String,
    pub number: i32,
}
