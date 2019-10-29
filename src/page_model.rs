use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseSNPedia {
    pub parse: Parse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parse {
    pub categories: Vec<Category>,
    pub displaytitle: String,
    pub externallinks: Vec<String>,
    pub images: Vec<String>,
    pub iwlinks: Vec<Value>,
    pub langlinks: Vec<Value>,
    pub links: Vec<Link>,
    pub pageid: i64,
    pub parsewarnings: Vec<Value>,
    pub properties: Vec<Property>,
    pub revid: i64,
    pub sections: Vec<Value>,
    pub templates: Vec<Template>,
    pub text: Text,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    #[serde(rename = "*")]
    pub field: String,
    pub sortkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "*")]
    pub field: String,
    pub exists: Option<String>,
    pub ns: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "*")]
    pub field: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Template {
    #[serde(rename = "*")]
    pub field: String,
    pub exists: String,
    pub ns: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Text {
    #[serde(rename = "*")]
    pub field: String,
}
