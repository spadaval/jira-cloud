/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeCreateBean {
    /// The unique name for the issue type. The maximum length is 60 characters.
    #[serde(rename = "name")]
    pub name: String,
    /// The description of the issue type.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the issue type is `subtype` or `standard`. Defaults to `standard`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
}

impl IssueTypeCreateBean {
    pub fn new(name: String) -> IssueTypeCreateBean {
        IssueTypeCreateBean {
            name,
            description: None,
            _type: None,
        }
    }
}

/// Whether the issue type is `subtype` or `standard`. Defaults to `standard`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "subtask")]
    Subtask,
    #[serde(rename = "standard")]
    Standard,
}