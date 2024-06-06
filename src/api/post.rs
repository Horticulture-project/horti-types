use serde::Serialize;

use super::ItemTypes;
#[derive(Debug, Clone, PartialEq, Serialize)]
struct ApiPost {
    #[serde(rename = "apiVersion")]
    api_version: String,
    data: ItemTypes,
}
impl ApiPost {}
#[derive(Debug, Clone, PartialEq, Serialize)]
struct ApiPostList {
    #[serde(rename = "apiVersion")]
    api_version: String,
    data: Vec<ItemTypes>,
}
