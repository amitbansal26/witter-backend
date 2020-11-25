use crate::tests::test_db::TestDatabase;
use crate::{create_server, State};
use serde_json::Value;
use std::collections::HashMap;
use tide::http::{Method, Request, Response, Url};
use tide::{Server, StatusCode};

pub struct TestServer {
    service: Server<State>,
    test_db: TestDatabase,
}

// Instantiation of Test Server
impl TestServer {
    fn new(service: Server<State>, test_db: TestDatabase) -> Self {
        Self { service, test_db }
    }
    pub async fn simulate(&self, req: Request) -> tide::Result<Response> {
        self.service.respond(req).await
    }
}

// Implement place holder methods for get put delete patch requests
#[derive(Debug)]
pub enum TestRequestType {
    Get,
    Post(Option<Value>),
    Delete,
}

#[derive(Debug)]
pub struct TestRequest {
    url: String,
    headers: HashMap<String, String>,
    kind: TestRequestType,
}

pub fn get(url: &str) -> TestRequest {
    TestRequest {
        url: url.to_string(),
        headers: HashMap::new(),
        kind: TestRequestType::Get,
    }
}

impl TestRequest {
    pub async fn send(self, server: &TestServer) -> (Value, StatusCode, HashMap<String, String>) {
        let url = Url::parse(&format!("http://example.com{}", self.url)).unwrap();
        let mut req = match self.kind {
            TestRequestType::Get => Request::new(Method::Get, url),
            TestRequestType::Post(Some(body)) => {
                let mut req = Request::new(Method::Post, url);
                req.set_body(body.to_string());
                req.set_content_type("application/json".parse().unwrap());
                req
            }
            TestRequestType::Delete => Request::new(Method::Delete, url),
            TestRequestType::Post(None) => panic!("Error condition Empty Post Body!!! Bhago !!!"),
        };

        let mut res = server.simulate(req).await.unwrap();
        let status = res.status();
        let headers = res
            .iter()
            .flat_map(|(key, values)| {
                values
                    .iter()
                    .map(move |value| (key.as_str().to_string(), value.as_str().to_string()))
            })
            .collect::<HashMap<_, _>>();
        let json = res.body_json::<Value>().await.unwrap();

        (json, status, headers)
    }
}
