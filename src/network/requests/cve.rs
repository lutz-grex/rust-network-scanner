use reqwest::Method;

use crate::models::cve_entry::CveEntry;
use crate::network::adapter::vulners::vulners_request::{VulnersRequest, VulnersRequestBuilder};
use crate::network::requests::network_request::NetworkRequest;

pub async fn request_cve(
    server: &str,
    score_min: f32,
) -> Result<Option<Vec<CveEntry>>, anyhow::Error> {
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

    match req.request().await {
        Ok(res) => {
            let mut filtered = res
                .data
                .search
                .iter()
                .filter_map(|cve_index| {
                    let cvss = cve_index.source.cvss.as_ref()?;
                    if cvss.score >= score_min {
                        Some(CveEntry {
                            title: cve_index.source.title.clone(),
                            description: cve_index.source.description.clone().unwrap_or_default(),
                            href: cve_index.source.href.clone().unwrap_or_default(),
                            score: cvss.score,
                            severity: cvss.severity.to_string(),
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            filtered.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
            Ok(Some(filtered))
        }
        Err(_) => Ok(None),
    }
}
