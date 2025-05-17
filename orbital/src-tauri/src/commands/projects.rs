use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, to_string};
use specta::Type;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::{AppHandle, Manager, Runtime, Wry};
use tauri_plugin_persistence::{
    types::ContextSpecifier, Context, FileHandle, FileHandleMode, PersistenceExt,
};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, sync::Mutex};

use crate::utils::{AppStateExt, Project};

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct ProjectCreationModel {
    pub name: String,
    pub asset_version: String,
}

#[taurpc::procedures(path = "projects")]
pub trait ProjectsApi {
    async fn projects_directory(app_handle: AppHandle<impl Runtime>) -> PathBuf;
    async fn create_project(
        app_handle: AppHandle<impl Runtime>,
        project: ProjectCreationModel,
    ) -> crate::Result<Project>;
    async fn open_project(
        app_handle: AppHandle<impl Runtime>,
        id: String,
    ) -> crate::Result<Project>;
    async fn delete_project(app_handle: AppHandle<impl Runtime>, id: String) -> crate::Result<()>;
    async fn active_project(app_handle: AppHandle<impl Runtime>) -> crate::Result<Option<Project>>;
}

pub struct ProjectsImpl<R: Runtime> {
    active: Arc<Mutex<Option<String>>>,
    context: Arc<Mutex<Option<Context<R>>>>,
}

impl<R: Runtime> Clone for ProjectsImpl<R> {
    fn clone(&self) -> Self {
        Self {
            active: self.active.clone(),
            context: self.context.clone(),
        }
    }
}

impl<R: Runtime> ProjectsImpl<R> {
    pub fn new() -> Self {
        Self {
            active: Arc::new(Mutex::new(None)),
            context: Arc::new(Mutex::new(None)),
        }
    }

    async fn proj_path<U: Runtime>(&self, handle: impl AppStateExt<U>) -> PathBuf {
        handle.app_state().await.project_path
    }

    async fn get_active(&self) -> Option<String> {
        self.active.lock().await.clone()
    }

    async fn set_active(&self, id: String) {
        let mut active = self.active.lock().await;
        active.insert(id);
    }

    async fn clear_active(&self) {
        let mut active = self.active.lock().await;
        let _ = active.take();
    }

    async fn active_context(&self) -> Option<Context<R>> {
        let ctx = self.context.lock().await.clone();
        if let Some(active) = ctx {
            Some(active)
        } else {
            None
        }
    }
}

#[taurpc::resolvers]
impl<R: Runtime> ProjectsApi for ProjectsImpl<R> {
    async fn projects_directory(self, app_handle: AppHandle<impl Runtime>) -> PathBuf {
        self.proj_path(app_handle).await
    }

    async fn create_project(
        self,
        app_handle: AppHandle<impl Runtime>,
        project: ProjectCreationModel,
    ) -> crate::Result<Project> {
        let persistence = app_handle.persistence();
        let project_id = project.name.to_case(Case::Snake);
        if let Some(ctx) = self.active_context().await {
            persistence
                .close_context(ContextSpecifier::Aliased { alias: ctx.name() })
                .await
                .or_else(|e| Err(crate::Error::from(e)))?;
        }

        let new_context = persistence
            .context(ContextSpecifier::Direct {
                alias: project_id.clone(),
                path: self
                    .proj_path(app_handle.clone())
                    .await
                    .join(project_id.clone()) /*  */
                    .to_str()
                    .unwrap()
                    .to_string(),
            })
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;

        let project_config = Project::new(project.name.clone(), project.asset_version.clone());

        let config_handle = new_context
            .open_file_handle(
                "project.json",
                FileHandleMode::Create {
                    new: true,
                    overwrite: true,
                },
            )
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;
        let mut file = config_handle.handle().await.lock();
        file.write_all(to_string(&project_config).unwrap().as_bytes())
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;
        file.flush().await.or_else(|e| Err(crate::Error::from(e)))?;
        Ok(project_config.clone())
    }
    async fn open_project(
        self,
        app_handle: AppHandle<impl Runtime>,
        id: String,
    ) -> crate::Result<Project> {
        let persistence = app_handle.persistence();
        if let Some(ctx) = self.active_context().await {
            persistence
                .close_context(ContextSpecifier::Aliased { alias: ctx.name() })
                .await
                .or_else(|e| Err(crate::Error::from(e)))?;
        }

        let new_context = persistence
            .context(ContextSpecifier::Direct {
                alias: id.clone(),
                path: self
                    .proj_path(app_handle.clone())
                    .await
                    .join(id.clone()) /*  */
                    .to_str()
                    .unwrap()
                    .to_string(),
            })
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;


        let config_handle = new_context
            .open_file_handle(
                "project.json",
                FileHandleMode::Read {  },
            )
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;
        let handle = config_handle.handle().await;
        let mut file = handle.lock();
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf).await.or_else(|e| Err(crate::Error::from(e)))?;
        let config = from_slice::<Project>(buf.as_slice()).or_else(|e| Err(crate::Error::deserialization(e)))?;
        Ok(config)
    }
    async fn delete_project(
        self,
        app_handle: AppHandle<impl Runtime>,
        id: String,
    ) -> crate::Result<()> {
    }
    async fn active_project(
        self,
        app_handle: AppHandle<impl Runtime>,
    ) -> crate::Result<Option<Project>> {
    }
}
