use serde::{
    Deserialize,
    Serialize,
};

/// Store options for Longrun and Oneshot
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ServiceOptions {
    pub dependencies: Vec<String>,
    pub requires: Vec<String>,
    pub requires_one: Vec<String>,
}

impl ServiceOptions {
    pub fn new() -> ServiceOptions {
        ServiceOptions {
            dependencies: Vec::new(),
            requires: Vec::new(),
            requires_one: Vec::new(),
        }
    }
}
impl Default for ServiceOptions {
    fn default() -> Self {
        Self::new()
    }
}
