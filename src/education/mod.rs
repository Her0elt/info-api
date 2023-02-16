use async_graphql::*;
use firestore::FirestoreDb;
use futures::stream::BoxStream;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

// Example structure to play with
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Education {
    id: String,
    from: String,
    to: String,
    school: String,
    title: String,
}

#[Object]
impl Education {
    pub async fn id(&self) -> String {
        return self.id.clone();
    }
    pub async fn from(&self) -> String {
        return self.from.clone();
    }
    pub async fn to(&self) -> String {
        return self.to.clone();
    }
    pub async fn school(&self) -> String {
        return self.school.clone();
    }
    pub async fn title(&self) -> String {
        return self.title.clone();
    }
}

impl Education {
    const EDUCATION_COLLECTION_NAME: &'static str = "education";
    pub async fn list() -> Vec<Self> {
        let db = FirestoreDb::new(String::from("cv-tracker-db866"))
            .await
            .unwrap();

        // Get a document as an object by id
        let objs_stream: BoxStream<Education> = db
            .fluent()
            .list()
            .from(Self::EDUCATION_COLLECTION_NAME)
            .obj()
            .stream_all()
            .await
            .unwrap();

        let docs: Vec<Education> = objs_stream.collect().await;
        return docs;
    }
}
