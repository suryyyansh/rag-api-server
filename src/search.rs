pub(crate) mod tavily_api;

use std::sync::Arc;
use async_trait::async_trait;
use once_cell::sync::OnceCell;
use hyper::{body::to_bytes, Body, Method, Request, Response};
use crate::error;

pub(crate) static CURRENT_SEARCH_API: OnceCell<Arc<dyn Query>> = OnceCell::new();

#[async_trait]
pub trait Query: Send + Sync {
    async fn search(&self, query: String) -> Result<String, Box<dyn std::error::Error>>;
}

pub async fn search_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    let mut resp: Option<String> = None;
    if req.method() == Method::POST {
        
        if let Some(ct) = req.headers().get("content-type") {
            if ct.to_str().unwrap_or("") == "application/json" {
                let req_body = req.into_body();
                let body_bytes = to_bytes(req_body).await?;
                if let Ok(json_body) = serde_json::from_slice::<serde_json::Value>(&body_bytes) {

                    let query = serde_json::to_string(&json_body["query"]).unwrap_or("".to_string());
                    resp = match CURRENT_SEARCH_API.get().expect("Search API not set.").search(query).await {
                        Ok(resp) => Some(resp),
                        Err(e) => return error::internal_server_error(e.to_string())
                    };
                }
            }
        }

        let result = Response::builder()
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "*")
            .header("Access-Control-Allow-Headers", "*")
            .header("Content-Type", "application/json")
            .body(Body::from(resp.unwrap_or("".to_string())));

        match result {
            Ok(response) => return Ok(response),
            Err(e) => {
                return error::internal_server_error(e.to_string());
            }
        }
    } else {
        return error::internal_server_error(format!("[SEARCH] Unsupported Method: {}", req.method()));
    }

} 

