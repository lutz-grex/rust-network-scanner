use crate::network::adapter::vulners::deserialize_models::{VulnersResponseStatus, VulnersResult};
use anyhow::anyhow;
use reqwest::{Client, Method};



#[derive(Debug)]
pub struct VulnersRequest<'a> {
    pub req_method: Method,
    pub version: &'a str,
    pub req_type: &'a str,
    pub search_engine: &'a str,
    pub query_params: Vec<(&'a str, &'a str)>,
}


#[derive(Debug)]
pub struct VulnersRequestBuilder<'a> {
    req_method: Option<Method>,
    version: Option<&'a str>,
    req_type: Option<&'a str>,
    search_engine: Option<&'a str>,
    query_params: Vec<(&'a str, &'a str)>,
}

impl<'a> VulnersRequestBuilder<'a> {
    pub fn new() -> Self {
        Self {
            req_method: None,
            version: None,
            req_type: None,
            search_engine: None,
            query_params: vec![],
        }
    }

    pub fn req_method(mut self, method: Method) -> Self {
        self.req_method = Some(method);
        self
    }

    pub fn version(mut self, version: &'a str) -> Self {
        self.version = Some(version);
        self
    }

    pub fn req_type(mut self, req_type: &'a str) -> Self {
        self.req_type = Some(req_type);
        self
    }

    pub fn search_engine(mut self, ka: &'a str) -> Self {
        self.search_engine = Some(ka);
        self
    }

    pub fn add_query_param(mut self, key: &'a str, value: &'a str) -> Self {
        self.query_params.push((key, value));
        self
    }

    pub fn build(self) -> Result<VulnersRequest<'a>, anyhow::Error> {
        Ok(VulnersRequest {
            req_method: self
                .req_method
                .ok_or_else(|| anyhow!("req_method is missing"))?,
            version: self.version.ok_or_else(|| anyhow!("missing version"))?,
            req_type: self.req_type.ok_or_else(|| anyhow!("req_type missing"))?,
            search_engine: self
                .search_engine
                .ok_or_else(|| anyhow!("missing search engine"))?,
            query_params: self.query_params,
        })
    }
}


