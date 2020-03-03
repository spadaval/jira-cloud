/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueFieldOptionConfiguration : Details of the projects the option is available in.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueFieldOptionConfiguration {
    /// Defines the projects that the option is available in. If the scope is not defined, then the option is available in all projects.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<crate::models::IssueFieldOptionScopeBean>,
    /// DEPRECATED
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Attributes>,
}

impl IssueFieldOptionConfiguration {
    /// Details of the projects the option is available in.
    pub fn new() -> IssueFieldOptionConfiguration {
        IssueFieldOptionConfiguration {
            scope: None,
            attributes: None,
        }
    }
}

/// DEPRECATED
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Attributes {
    #[serde(rename = "notSelectable")]
    NotSelectable,
    #[serde(rename = "defaultValue")]
    DefaultValue,
}