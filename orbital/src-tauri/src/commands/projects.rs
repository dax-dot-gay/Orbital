use serde::{Deserialize, Serialize};
use specta::Type;
use std::sync::Arc;
use tauri::{AppHandle, Runtime};
use tokio::sync::Mutex;

use crate::utils::{Project, ProjectConfig, ProjectsExt};

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct ProjectCreationModel {
    pub name: String,
    pub asset_version: String,
}

#[taurpc::procedures(path = "projects")]
pub trait ProjectsApi {
    async fn list_projects(app_handle: AppHandle<impl Runtime>) -> crate::Result<Vec<ProjectConfig>>;
}

pub struct ProjectsImpl<R: Runtime> {
    active: Arc<Mutex<Option<String>>>,
    project: Arc<Mutex<Option<Project<R>>>>,
}

impl<R: Runtime> Clone for ProjectsImpl<R> {
    fn clone(&self) -> Self {
        Self {
            active: self.active.clone(),
            project: self.project.clone(),
        }
    }
}

impl<R: Runtime> ProjectsImpl<R> {
    pub fn new() -> Self {
        Self {
            active: Arc::new(Mutex::new(None)),
            project: Arc::new(Mutex::new(None)),
        }
    }

    
}

#[taurpc::resolvers]
impl<R: Runtime> ProjectsApi for ProjectsImpl<R> {
    async fn list_projects(self, app_handle: AppHandle<impl Runtime>) -> crate::Result<Vec<ProjectConfig>> {
        app_handle.list_projects().await
    }
}
