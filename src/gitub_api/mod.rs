use std::collections::HashMap;
use serde::Deserialize;
use gql_client::Client;
use async_graphql::*;

#[derive(Deserialize, Debug)]
struct ApiResponse<T> {
    data: T
}

#[derive(Deserialize, Debug)]
struct Data {
    viewer: Viewer,
}

#[derive(Deserialize, Debug)]
struct Viewer {
    pinnedItems: PinnedItems,
}

#[derive(Deserialize, Debug)]
struct PinnedItems {
    nodes : Vec<Repository>,
}


#[derive(Deserialize, Debug)]
pub struct Repository {
    name: String,
    description: Option<String>,
    url: Option<String>,
}

#[Object]
impl Repository {
    pub async fn name(&self) -> String {
        return self.name.clone();
    }
    pub async fn description(&self) -> Option<String> {
        return self.description.clone();
    }
    pub async fn url(&self) -> Option<String> {
        return self.url.clone();
    }

}

impl Repository {

    const API_URL: &'static str = "https://api.github.com/graphql";
    const QUERY_STRING: &'static str = "
            query {
                viewer {
                    pinnedItems(first:10) {
                        nodes {
                            ... on Repository {
                                name
                                description
                                url
                            }
                        }
                    }
                }
            }
        ";
    pub async fn list() -> Vec<Self> {
        let api_token = std::env::var("GITHUB_API_TOKEN").expect("Github API token has to be included");
        let mut headers = HashMap::new();
        headers.insert("authorization", format!("Bearer {}", api_token));
        headers.insert("accept", String::from("application/json"));
        headers.insert("User-Agent", String::from("info-api"));


        let client = Client::new_with_headers(Self::API_URL, headers);
        let response: Data = client.query(Self::QUERY_STRING).await.unwrap().unwrap();
        let repos = response.viewer.pinnedItems.nodes;
        return repos;
    }
}

