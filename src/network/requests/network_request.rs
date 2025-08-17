

#[async_trait::async_trait]
pub trait NetworkRequest {
    type Response;
    
    const BASE_URL: &'static str;

    async fn request(&self) -> Result<Self::Response, anyhow::Error>;
}