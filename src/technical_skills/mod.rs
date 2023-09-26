use std::collections::BTreeMap;

use async_graphql::*;
use firestore::FirestoreDb;
use futures::stream::BoxStream;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

// Example structure to play with
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TechnicalSkills {
    id: String,
    category: String,
    skill: String,
}

#[Object]
impl TechnicalSkills {
    pub async fn id(&self) -> String {
        return self.id.clone();
    }
    pub async fn category(&self) -> String {
        return self.category.clone();
    }
    pub async fn skill(&self) -> String {
        return self.skill.clone();
    }
}

impl TechnicalSkills {
    const TECHNICALSKILLS_COLLECTION_NAME: &'static str = "technical-skills";

    pub async fn list() -> Vec<Self> {
        let project_id = std::env::var("GCP_PROJECT_ID").expect("Need to provide gcp project id ");
        let db = FirestoreDb::new(project_id).await.unwrap();

        // Get a document as an object by id
        let objs_stream: BoxStream<TechnicalSkills> = db
            .fluent()
            .list()
            .from(Self::TECHNICALSKILLS_COLLECTION_NAME)
            .obj()
            .stream_all()
            .await
            .unwrap();

        let docs: Vec<TechnicalSkills> = objs_stream.collect().await;
        return docs;
    }
    pub async fn group_by_category() -> BTreeMap<String, Vec<Self>> {
        let project_id = std::env::var("GCP_PROJECT_ID").expect("Need to provide gcp project id ");
        let db = FirestoreDb::new(project_id).await.unwrap();

        // Get a document as an object by id
        let objs_stream: BoxStream<TechnicalSkills> = db
            .fluent()
            .list()
            .from(Self::TECHNICALSKILLS_COLLECTION_NAME)
            .obj()
            .stream_all()
            .await
            .unwrap();

        let docs: Vec<TechnicalSkills> = objs_stream.collect().await;
        let mut groups: BTreeMap<String, Vec<Self>> = BTreeMap::new();

        for item in docs {
            let field_val = item.category.clone();
            groups.entry(field_val).or_insert_with(Vec::new).push(item);
        }
        return groups;
    }
}
