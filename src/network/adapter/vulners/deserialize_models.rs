use core::fmt;

use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct VulnersResult {
    pub result: VulnersResponseStatus,
    pub data: VulnersSearchResult,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum VulnersResponseStatus {
    OK,
    FAILED,
}

#[derive(Debug, Deserialize)]
pub struct VulnersSearchResult {
    #[serde(default)]
    pub search: Vec<VulnersCveEntry>,
    
    #[serde(rename = "exactMatch", default)]
    pub exact_match: Option<serde_json::Value>,
    
    #[serde(default)]
    pub occurrences: Option<serde_json::Value>,
    
    #[serde(default)]
    pub references: Option<serde_json::Value>,
    
    #[serde(default)]
    pub total: Option<u32>,
    
    #[serde(rename = "maxSearchSize", default)]
    pub max_search_size: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct VulnersCveEntry {
    #[serde(rename = "_source")]
    pub source: VulnersSource,
}

#[derive(Debug, Deserialize)]
pub struct VulnersSource {
    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub title: String,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub published: Option<String>,

    #[serde(default)]
    pub href: Option<String>,

    #[serde(default)]
    pub vhref: Option<String>,

    #[serde(default)]
    pub cvss: Option<Cvss>,
}

#[derive(Debug, Deserialize)]
pub struct Cvss {
    #[serde(default)]
    pub score: f32,

    #[serde()]
    pub severity: SeverityScore,

    #[serde(default)]
    pub version: String,

    #[serde(default)]
    pub vector: String,

    #[serde(default)]
    pub source: String,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SeverityScore {
    SECURITY_WARNING,
    CRITICAL,
    HIGH,
    MEDIUM,
    LOW,
    NONE,
    UNKNOWN
}

/**
 * to call to_string on enum
 */
impl fmt::Display for SeverityScore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SeverityScore::SECURITY_WARNING => write!(f, "SECURITY_WARNING"),
            SeverityScore::CRITICAL => write!(f, "CRITICAL"),
            SeverityScore::HIGH => write!(f, "HIGH"),
            SeverityScore::MEDIUM => write!(f, "MEDIUM"),
            SeverityScore::LOW => write!(f, "LOW"),
            SeverityScore::NONE => write!(f, "NONE"),
            SeverityScore::UNKNOWN => write!(f, "UNKNOWN"),
        }
    }
}