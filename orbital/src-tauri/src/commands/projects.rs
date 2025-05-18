use serde::{Deserialize, Serialize};
use specta::Type;
use std::sync::Arc;
use tauri::{ipc::CommandArg, AppHandle, Manager, Runtime, Wry};
use tokio::sync::Mutex;

use crate::utils::{Project, ProjectConfig, ProjectsExt};

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct ProjectCreationModel {
    pub name: String,
    pub asset_version: String,
}

#[taurpc::procedures(path = "projects")]
pub trait ProjectsApi {
    async fn list_projects(app_handle: AppHandle) -> crate::Result<Vec<ProjectConfig>>;
    async fn create_project(app_handle: AppHandle, model: ProjectCreationModel) -> crate::Result<ProjectConfig>;
    async fn open_project(app_handle: AppHandle, id: String) -> crate::Result<ProjectConfig>;
    async fn close_project(app_handle: AppHandle) -> crate::Result<()>;
    async fn remove_project(app_handle: AppHandle, id: String) -> crate::Result<()>;
    async fn current_project(app_handle: AppHandle) -> crate::Result<Option<ProjectConfig>>;
    async fn project_config(app_handle: AppHandle, id: String) -> crate::Result<ProjectConfig>;
}

pub struct ProjectsImpl {
    active: Arc<Mutex<Option<String>>>,
    project: Arc<Mutex<Option<Project>>>,
}

impl Clone for ProjectsImpl {
    fn clone(&self) -> Self {
        Self {
            active: self.active.clone(),
            project: self.project.clone(),
        }
    }
}

impl ProjectsImpl {
    pub fn new() -> Self {
        Self {
            active: Arc::new(Mutex::new(None)),
            project: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn set_active(&self, project: Project) -> () {
        let mut active = self.active.lock().await;
        let mut project_ref = self.project.lock().await;

        active.insert(project.id());
        project_ref.insert(project);
    }

    pub async fn clear_active(&self) -> crate::Result<()> {
        let mut active = self.active.lock().await;
        let mut project_ref = self.project.lock().await;

        let _ = active.take();
        let existing = project_ref.take();
        if existing.is_some() {
            existing.unwrap().close().await?;
        }

        Ok(())
    }
}

#[taurpc::resolvers]
impl ProjectsApi for ProjectsImpl {
    async fn list_projects(self, app_handle: AppHandle<impl Runtime>) -> crate::Result<Vec<ProjectConfig>> {
        app_handle.list_projects().await
    }

    async fn create_project(self, app_handle: AppHandle<impl Runtime>, model: ProjectCreationModel) -> crate::Result<ProjectConfig> {
        let new_config = ProjectConfig::new(model.name, model.asset_version);
        let created_project = app_handle.create_project(new_config.clone()).await?;
        self.clear_active().await?;
        self.set_active(created_project).await;
        Ok(new_config)

    }
    async fn open_project(self, app_handle: AppHandle<impl Runtime>, id: String) -> crate::Result<ProjectConfig> {

    }
    async fn close_project(self, app_handle: AppHandle<impl Runtime>) -> crate::Result<()> {

    }
    async fn remove_project(self, app_handle: AppHandle<impl Runtime>, id: String) -> crate::Result<()> {

    }
    async fn current_project(self, app_handle: AppHandle<impl Runtime>) -> crate::Result<Option<ProjectConfig>> {

    }
    async fn project_config(self, app_handle: AppHandle<impl Runtime>, id: String) -> crate::Result<ProjectConfig> {

    }
}
