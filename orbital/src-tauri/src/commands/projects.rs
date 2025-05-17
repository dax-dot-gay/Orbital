use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::{AppHandle, Manager, Runtime};
use tokio::sync::Mutex;

use crate::utils::AppStateExt;

#[taurpc::procedures(path = "projects")]
pub trait ProjectsApi {
    async fn project_path(app_handle: AppHandle<impl Runtime>) -> PathBuf;
}

#[derive(Clone)]
pub struct ProjectsImpl;

impl ProjectsImpl {
    async fn proj_path<R: Runtime>(&self, handle: impl AppStateExt<R>) -> PathBuf {
        handle.app_state().await.project_path
    }
}

#[taurpc::resolvers]
impl ProjectsApi for ProjectsImpl {
    async fn project_path(self, app_handle: AppHandle<impl Runtime>) -> PathBuf {
        self.proj_path(app_handle).await
    }
}
