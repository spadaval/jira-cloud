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
pub struct UserWriteBean {
    /// The URL of the user.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
    /// The key for the user. When provided with `name`, overrides the value in `name` to set both `name` and `key`. This property is deprecated because of privacy changes. See the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The username for the user. This property is deprecated because of privacy changes. See the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A password for the user. If a password is not set, a random password is generated.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// The email address for the user.
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    /// The display name for the user.
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// Sends the user an email confirmation that they have been added to Jira. Default is `false`.
    #[serde(rename = "notification", skip_serializing_if = "Option::is_none")]
    pub notification: Option<String>,
    /// Deprecated, do not use.
    #[serde(rename = "applicationKeys", skip_serializing_if = "Option::is_none")]
    pub application_keys: Option<Vec<String>>,
}

impl UserWriteBean {
    pub fn new(email_address: String, display_name: String) -> UserWriteBean {
        UserWriteBean {
            _self: None,
            key: None,
            name: None,
            password: None,
            email_address,
            display_name,
            notification: None,
            application_keys: None,
        }
    }
}