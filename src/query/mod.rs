use crate::education::Education;
use crate::job_experience::JobExperience;
use crate::technical_skills::TechnicalSkills;
use async_graphql::*;

// Example structure to play with

pub struct Query;

#[Object]
impl Query {
    pub async fn educations(&self) -> Vec<Education> {
        return Education::list().await;
    }
    pub async fn job_experiences(&self) -> Vec<JobExperience> {
        return JobExperience::list().await;
    }
    pub async fn technical_skills(&self) -> Vec<TechnicalSkills> {
        return TechnicalSkills::list().await;
    }
}
