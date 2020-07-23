use crate::dal::yarn_api::YarnApi;
use crate::model::clip::Clip;
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

#[async_trait]
pub trait ClipDao: Sync + Send {
    async fn query(&self, query: &str) -> Result<Vec<Clip>, Box<dyn Error>>;
}

#[derive(Clone)]
pub struct ClipDaoImpl {
    yarn_api: Arc<dyn YarnApi>,
}

#[async_trait]
impl ClipDao for ClipDaoImpl {
    async fn query(&self, query: &str) -> Result<Vec<Clip>, Box<dyn Error>> {
        let clips = self.yarn_api.query(query).await?;
        Ok(clips.iter().map(Clip::from).collect())
    }
}

impl ClipDaoImpl {
    pub fn new(yarn_api: Arc<dyn YarnApi>) -> ClipDaoImpl {
        ClipDaoImpl { yarn_api }
    }
}
