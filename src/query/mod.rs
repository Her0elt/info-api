use std::collections::BTreeMap;

use crate::education::Education;
use crate::job_experience::JobExperience;
use crate::technical_skills::TechnicalSkills;
use crate::gitub_api::Repository;
use async_graphql::*;

// Example structure to play with

pub struct Query;

#[Object]
impl Query {
    pub async fn health_check(&self) -> &str {
        return "awake and ready"
    }
    pub async fn educations(&self) -> Vec<Education> {
        return Education::list().await;
    }
    pub async fn job_experiences(&self) -> Vec<JobExperience> {
        return JobExperience::list().await;
    }
    pub async fn technical_skills(&self) -> Vec<TechnicalSkills> {
        return TechnicalSkills::list().await;
    }
    pub async fn group_by_category(&self) -> BTreeMap<String, Vec<TechnicalSkills>> {
        return TechnicalSkills::group_by_category().await;
    }
    pub async fn pinns(&self) -> Vec<Repository> {
        return Repository::list().await;
    }


}
