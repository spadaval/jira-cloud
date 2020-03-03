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

pub struct JiraSettingsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl JiraSettingsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> JiraSettingsApiClient {
        JiraSettingsApiClient { configuration }
    }
}

pub trait JiraSettingsApi {
    fn get_advanced_settings(&self) -> Result<Vec<crate::models::ApplicationProperty>, Error>;
    fn get_application_property(
        &self,
        key: Option<&str>,
        permission_level: Option<&str>,
        key_filter: Option<&str>,
    ) -> Result<Vec<crate::models::ApplicationProperty>, Error>;
    fn get_configuration(&self) -> Result<crate::models::Configuration, Error>;
    fn set_application_property_put(
        &self,
        id: &str,
        simple_application_property_bean: crate::models::SimpleApplicationPropertyBean,
    ) -> Result<crate::models::ApplicationProperty, Error>;
}

impl JiraSettingsApi for JiraSettingsApiClient {
    fn get_advanced_settings(&self) -> Result<Vec<crate::models::ApplicationProperty>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/application-properties/advanced-settings",
            configuration.base_path
        );
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

    fn get_application_property(
        &self,
        key: Option<&str>,
        permission_level: Option<&str>,
        key_filter: Option<&str>,
    ) -> Result<Vec<crate::models::ApplicationProperty>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/application-properties",
            configuration.base_path
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = key {
            req_builder = req_builder.query(&[("key", &s.to_string())]);
        }
        if let Some(ref s) = permission_level {
            req_builder = req_builder.query(&[("permissionLevel", &s.to_string())]);
        }
        if let Some(ref s) = key_filter {
            req_builder = req_builder.query(&[("keyFilter", &s.to_string())]);
        }
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

    fn get_configuration(&self) -> Result<crate::models::Configuration, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/rest/api/3/configuration", configuration.base_path);
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

    fn set_application_property_put(
        &self,
        id: &str,
        simple_application_property_bean: crate::models::SimpleApplicationPropertyBean,
    ) -> Result<crate::models::ApplicationProperty, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/application-properties/{id}",
            configuration.base_path,
            id = crate::apis::urlencode(id)
        );
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };
        req_builder = req_builder.json(&simple_application_property_bean);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}