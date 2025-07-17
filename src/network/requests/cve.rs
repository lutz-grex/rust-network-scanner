use reqwest::Method;
use std::error::Error;

use crate::{models::connection::CveEntry, network::adapter::vulners::connector::{request, VulnersRequestBuilder}};




pub async fn request_cve(server: &str, score_min: f32) ->  Result<Option<Vec<CveEntry>> , Box<dyn Error + Send + Sync>> {

    if server.is_empty() || score_min > 10.0 {
        return Ok(None);
    }

    let req = VulnersRequestBuilder::new()
        .req_method(Method::GET)
        .version("v3")
        .req_type("search")
        .search_engine("lucene")
        .add_query_param("query", server)
        .build()?;

        println!("{:?}", req);

    match request(&req).await {
        Ok(res) => {
            let mut filtered = res.data.search.iter()
                .filter(|cve_index| {
                    cve_index.source.cvss
                        .as_ref()
                        .map_or(false, |cvss| cvss.score >= score_min)
                })
                .map(|c| {
                    let cvss = c.source.cvss.as_ref();
                    CveEntry {
                        title: c.source.title.clone(),
                        description: c.source.description.clone().unwrap_or_default(),
                        href: c.source.href.clone().unwrap_or_default(),
                        score: cvss.map_or(0.0, |c| c.score),
                        severity: cvss.map_or("UNKNOWN".to_string(), |c| c.severity.clone().to_string()),
                    }
                })
                .collect::<Vec<_>>();

            filtered.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
            Ok(Some(filtered))
        }
        Err(_) => Ok(None),
    }
}