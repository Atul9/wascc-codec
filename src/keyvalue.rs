//! # Key-Value Store Data Types
//!
//! This module contains data types for the `wascc:keyvalue` capability provider

use crate::Sample;

pub const OP_ADD: &str = "Add";
pub const OP_GET: &str = "Get";
pub const OP_SET: &str = "Set";
pub const OP_DEL: &str = "Del";
pub const OP_CLEAR: &str = "Clear";
pub const OP_RANGE: &str = "Range";
pub const OP_PUSH: &str = "Push";
pub const OP_LIST_DEL: &str = "ListItemDelete";

pub const OP_SET_ADD: &str = "SetAdd";
pub const OP_SET_REMOVE: &str = "SetRemove";
pub const OP_SET_UNION: &str = "SetUnion";
pub const OP_SET_INTERSECT: &str = "SetIntersection";
pub const OP_SET_QUERY: &str = "SetQuery";
pub const OP_KEY_EXISTS: &str = "KeyExists";

/// A request to get a single value from the K/V store
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRequest {
    pub key: String,
}

/// The result of a get request
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetResponse {
    /// The value returned from the data store
    pub value: String,
    /// Indicates whether the key existed
    pub exists: bool,
}

/// A request to set a value
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetRequest {
    /// Key of the item to set
    pub key: String,
    /// Value of the item to set
    pub value: String,
    /// Seconds after which the key will expire, 0 - no expiration
    #[serde(rename = "expires")]
    pub expires_s: i32,
}

impl Sample for SetRequest {
    fn sample() -> Self {
        SetRequest {
            key: "counter1".to_string(),
            value: "12".to_string(),
            expires_s: 100,
        }
    }
}

/// A request to delete a key
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DelRequest {
    pub key: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DelResponse {
    pub key: String,
}

/// Response to a set request
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetResponse {
    pub value: String,
}

/// A request to perform an atomic add operation
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRequest {
    pub key: String,
    pub value: i32,
}

/// Result of an atomic add operation
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddResponse {
    pub value: i32,
}

/// A request to add an item to the end of a list
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPushRequest {
    pub key: String,
    pub value: String,
}

/// A request to delete all occurences of an item from a list
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListDelItemRequest {
    pub key: String,
    pub value: String,
}

/// A request to clear a list at a given key
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListClearRequest {
    pub key: String,
}

/// A request to retrieve a range of values from a list
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRangeRequest {
    pub key: String,
    pub start: i32,
    pub stop: i32,
}

/// List of values returned from a range request
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRangeResponse {
    #[serde(default)]
    pub values: Vec<String>,
}

/// Return response from non-range list requests like push and clear
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse {
    pub new_count: i32,
}

/// Request to add an item to a set
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAddRequest {
    pub key: String,
    pub value: String,
}

/// Request to remove a specific value from a set
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetRemoveRequest {
    pub key: String,
    pub value: String,
}

/// Request to query the contents of a set
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetQueryRequest {
    pub key: String,
}

/// Response to an operation that requests members of a list (query, intersect, union)
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetQueryResponse {
    #[serde(default)]
    pub values: Vec<String>,
}

/// Request for the intersection of multiple sets
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetIntersectionRequest {
    #[serde(default)]
    pub keys: Vec<String>,
}

/// Request for the union of multiple sets
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetUnionRequest {
    #[serde(default)]
    pub keys: Vec<String>,
}

/// Response to a set query, add, or delete
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetOperationResponse {
    pub new_count: i32,
}

/// Test for the existence of a key
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyExistsQuery {
    pub key: String,
}
