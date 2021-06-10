use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MyJsonFile {
    pub name: String,
    pub number: u32,
}
