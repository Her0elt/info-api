use async_graphql::*;
use gql_client::Client;
use serde::Deserialize;
use std::collections::HashMap;

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
    nodes: Vec<Repository>,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    id: String,
    name: String,
    description: String,
    url: String,
    languages: Languages,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Languages {
    nodes: Vec<Language>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Language {
    name: String,
}

#[Object]
impl Repository {
    pub async fn id(&self) -> String {
        return self.id.clone();
    }
    pub async fn name(&self) -> String {
        return self.name.clone();
    }
    pub async fn description(&self) -> String {
        return self.description.clone();
    }
    pub async fn url(&self) -> String {
        return self.url.clone();
    }

    pub async fn languages(&self) -> Languages {
        return self.languages.clone();
    }
}

#[Object]
impl Languages {
    pub async fn nodes(&self) -> Vec<Language> {
        return self.nodes.clone();
    }
}

#[Object]
impl Language {
    pub async fn name(&self) -> String {
        return self.name.clone();
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
                                id
                                name
                                description
                                url
                                languages(first: 3) {
                                            nodes {
                                            name
                                            }
                                            
                                        }
                            }
                        }
                    }
                }
            }
        ";
    pub async fn list() -> Vec<Self> {
        let api_token =
            std::env::var("GITHUB_API_TOKEN").expect("Github API token has to be included");
        let mut headers = HashMap::new();
        headers.insert("authorization", format!("Bearer {}", api_token));
        headers.insert("accept", String::from("application/json"));
        headers.insert("User-Agent", String::from("info-api"));

        let client = Client::new_with_headers(Self::API_URL, headers);
        let response = client
            .query::<Data>(Self::QUERY_STRING)
            .await
            .unwrap()
            .unwrap();
        let repos = response.viewer.pinnedItems.nodes;
        return repos;
    }
}
