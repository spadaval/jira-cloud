/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::rc::Rc;

use reqwest;

use super::{configuration, Error};

pub struct IssueNavigatorSettingsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl IssueNavigatorSettingsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> IssueNavigatorSettingsApiClient {
        IssueNavigatorSettingsApiClient { configuration }
    }
}

pub trait IssueNavigatorSettingsApi {
    fn get_issue_navigator_default_columns(&self) -> Result<Vec<crate::models::ColumnItem>, Error>;
    fn set_issue_navigator_default_columns_put(
        &self,
        request_body: Option<Vec<String>>,
    ) -> Result<serde_json::Value, Error>;
}

impl IssueNavigatorSettingsApi for IssueNavigatorSettingsApiClient {
    fn get_issue_navigator_default_columns(&self) -> Result<Vec<crate::models::ColumnItem>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/rest/api/3/settings/columns", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn set_issue_navigator_default_columns_put(
        &self,
        request_body: Option<Vec<String>>,
    ) -> Result<serde_json::Value, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/rest/api/3/settings/columns", configuration.base_path);
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };
        req_builder = req_builder.json(&request_body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}