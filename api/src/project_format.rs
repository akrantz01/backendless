use serde::{Deserialize, Serialize};
use serde_json::value::Value;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectFormat {
    pub name: String,
    #[serde(default)]
    pub version: String,
    pub static_directory: String,
    pub routes: Vec<Route>,
    pub handlers: Vec<Handler>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Route {
    pub path: String,
    pub methods: Vec<String>,
    pub handler: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Handler {
    pub name: String,
    pub query_parameters: Option<Vec<String>>,
    pub headers: Option<Vec<String>>,
    pub path_parameters: Option<Vec<String>>,
    pub body: Option<Value>,
    pub logic: Value,
}
