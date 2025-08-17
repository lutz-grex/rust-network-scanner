use anyhow::anyhow;
use reqwest::{Client, Method};
use crate::network::adapter::vulners::vulners_request::VulnersRequest;
use crate::network::adapter::vulners::deserialize_models::{VulnersResponseStatus, VulnersResult};
use crate::network::requests::network_request::NetworkRequest;



#[async_trait::async_trait]
impl<'a> NetworkRequest for VulnersRequest<'a> {
    type Response = VulnersResult;

    const BASE_URL: &'static str = "https://vulners.com/api";

    async fn request(&self) -> Result<Self::Response, anyhow::Error> {
        let client = Client::new();

        let url = format!(
            "{}/{}/{}/{}",
            Self::BASE_URL, self.version, self.req_type, self.search_engine
        );

        #[cfg(debug_assertions)]
        {
            println!("Request URL: {}", url);
            println!("Query params: {:?}", self.query_params);
        }

        match self.req_method {
            Method::GET => {
                let res = client.get(&url).query(&self.query_params).send().await?;

                let vulners_result: VulnersResult = res.json().await.map_err(|e| {
                    eprintln!("JSON parsing error: {}", e);
                    anyhow!("Failed to parse JSON response")
                })?;

                match vulners_result.result {
                    VulnersResponseStatus::OK => {
                        #[cfg(debug_assertions)]
                        println!("Success: {:?}", vulners_result);
                        Ok(vulners_result)
                    }
                    _ => Err(anyhow!("Invalid API response: {:?}", vulners_result)),
                }
            }
            _ => Err(anyhow!("HTTP method not implemented: {:?}", self.req_method)),
        }
    }
}