use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod utils;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromClient {
    Join {
        group_name: Arc<String>,
    },
    Post {
        group_name: Arc<String>,
        message: Arc<String>,
    },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromServer {
    Message {
        group_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}

#[test]
fn test_from_client_json() {
    let from_client = FromClient::Post {
        group_name: Arc::new("Dogs".into()),
        message: Arc::new("Basenjis rule!".into()),
    };
    let post = serde_json::to_value(&from_client).unwrap();

    assert_eq!(
        post,
        serde_json::json!({ "Post": { "group_name": "Dogs", "message": "Basenjis rule!" } })
    );
    assert_eq!(from_client, serde_json::from_value(post).unwrap());
}
