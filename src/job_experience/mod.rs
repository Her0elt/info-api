use async_graphql::*;
use firestore::FirestoreDb;
use futures::stream::BoxStream;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

// Example structure to play with
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobExperience {
    id: String,
    from: String,
    to: String,
    company: String,
    description: String,
    title: String,
}

#[Object]
impl JobExperience {
    pub async fn id(&self) -> String {
        return self.id.clone();
    }
    pub async fn from(&self) -> String {
        return self.from.clone();
    }
    pub async fn to(&self) -> String {
        return self.to.clone();
    }
    pub async fn description(&self) -> String {
        return self.description.clone();
    }
    pub async fn company(&self) -> String {
        return self.company.clone();
    }
    pub async fn title(&self) -> String {
        return self.title.clone();
    }
}

impl JobExperience {
    const JOBEXPERIENCE_COLLECTION_NAME: &'static str = "job-experience";
    pub async fn list() -> Vec<Self> {
        let db = FirestoreDb::new(String::from("cv-tracker-db866"))
            .await
            .unwrap();

        // Get a document as an object by id
        let objs_stream: BoxStream<JobExperience> = db
            .fluent()
            .list()
            .from(Self::JOBEXPERIENCE_COLLECTION_NAME)
            .obj()
            .stream_all()
            .await
            .unwrap();

        let docs: Vec<JobExperience> = objs_stream.collect().await;
        return docs;
    }
}
