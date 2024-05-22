use hyper::{Body, Client, Method, Request};
use crate::search::Query;
use serde_json::json;
use async_trait::async_trait;

const SEARCH_ENDPOINT: &str = "http://api.tavily.com/search";

pub struct TavilyAPISearch {
    pub api_key: String,
    pub max_search_results: i32,
}

#[async_trait]
impl Query for TavilyAPISearch {
    async fn search(&self, query: String) -> Result<String, Box<dyn std::error::Error>> {
        let client = Client::new();

        let req = Request::builder()
            .method(Method::POST)
            .uri(SEARCH_ENDPOINT)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({
                    "api_key": self.api_key,
                    "query": query,
                    "include_images": false,
                    "max_search_results": self.max_search_results
                })
                .to_string()
            ))?;

        let resp = client.request(req).await?;
        let bytes = hyper::body::to_bytes(resp.into_body()).await?;
        let body_json: serde_json::Value = serde_json::from_slice(&bytes)?;
        
        Ok(serde_json::to_string_pretty(&body_json["results"])?)
    }
}
