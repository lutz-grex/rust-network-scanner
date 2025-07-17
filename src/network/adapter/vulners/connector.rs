use reqwest::{Client, Method};
use crate::network::adapter::vulners::deserialize_models::{VulnersResponseStatus, VulnersResult};


const BASE_URL: &str = "https://vulners.com/api";

#[derive(Debug)]
pub struct VulnersRequest<'a> {
    pub req_method: Method,
    pub version: &'a str,
    pub req_type: &'a str,
    pub search_engine: &'a str,
    pub query_params: Vec<(&'a str, &'a str)>
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
            query_params: Vec::new(),
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

    pub fn build(self) -> Result<VulnersRequest<'a>, &'static str> {
        Ok(VulnersRequest {
            req_method: self.req_method.ok_or("req_method is missing")?,
            version: self.version.unwrap_or("v3"),
            req_type: self.req_type.ok_or("req_type is missing")?,
            search_engine: self.search_engine.unwrap_or("lucene"),
            query_params: self.query_params,
        })
    }
}




pub async fn request<'a>(request: &VulnersRequest<'a>) -> Result<VulnersResult, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("{BASE_URL}/{}/{}/{}", request.version, request.req_type, request.search_engine);
    
    #[cfg(debug_assertions)]
    {
        println!("Request URL: {}", url);
        println!("Query params: {:?}", request.query_params);
    }
    
    match request.req_method {
        Method::GET => {
            let req = client.get(&url).query(&request.query_params);
            let res = req.send().await?;

            let vulners_result: VulnersResult = res.json().await
                .map_err(|e| {
                    eprintln!("JSON parsing error: {}", e);
                    Box::new(e) as Box<dyn std::error::Error>
            })?;

            match vulners_result.result {
                VulnersResponseStatus::OK => {
                    #[cfg(debug_assertions)]
                    println!("Success: {:?}", vulners_result);
                    
                    Ok(vulners_result)
                }
                _ => {
                    eprintln!("API Error: {:?}", vulners_result);
                    Err("Invalid API response".into())
                }
            }
        },
        _ => Err("Not implemented".into())
    }
}
