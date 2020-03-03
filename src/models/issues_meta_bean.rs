/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssuesMetaBean : Meta data describing the `issues` context variable.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssuesMetaBean {
    #[serde(rename = "jql", skip_serializing_if = "Option::is_none")]
    pub jql: Option<crate::models::IssuesJqlMetaDataBean>,
}

impl IssuesMetaBean {
    /// Meta data describing the `issues` context variable.
    pub fn new() -> IssuesMetaBean {
        IssuesMetaBean { jql: None }
    }
}