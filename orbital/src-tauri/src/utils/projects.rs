use std::{fs, path::PathBuf};

use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, to_string_pretty};
use specta::Type;
use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_persistence::{
    types::ContextSpecifier, Context, Database, FileHandleMode, PersistenceExt,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::AppStateExt;

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct ProjectConfig {
    pub id: String,
    pub name: String,
    pub asset_version: String,
}

impl ProjectConfig {
    pub fn new(name: impl AsRef<str>, asset_version: impl AsRef<str>) -> Self {
        let name = name.as_ref().to_string();
        let asset_version = asset_version.as_ref().to_string();
        Self {
            id: name.to_case(Case::Snake),
            name,
            asset_version,
        }
    }
}

pub struct Project {
    context: Context<Wry>,
    config: ProjectConfig,
    handle: AppHandle<Wry>,
}

impl Clone for Project {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            config: self.config.clone(),
            handle: self.handle.clone(),
        }
    }
}

impl Project {
    pub async fn projects_directory(&self) -> PathBuf {
        self.handle.app_state().await.project_path
    }

    pub async fn directory(&self) -> PathBuf {
        self.projects_directory().await.join(self.config.id.clone())
    }

    pub fn config(&self) -> ProjectConfig {
        self.config.clone()
    }

    pub async fn update_config(&mut self, conf: ProjectConfig) -> crate::Result<ProjectConfig> {
        self.config = conf.clone();

        let conf_handle = self
            .context
            .open_file_handle("project.json", FileHandleMode::overwrite())
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;
        let conf_locked = conf_handle.handle().await;
        let mut config_file = conf_locked.lock();

        config_file
            .write_all(to_string_pretty(&self.config).unwrap().as_bytes())
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;
        config_file
            .flush()
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;
        conf_handle
            .close()
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;

        Ok(self.config.clone())
    }

    pub async fn close(self) -> crate::Result<()> {
        self.context
            .close()
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;
        Ok(())
    }

    pub async fn database(&self) -> crate::Result<Database<Wry>> {
        self.context
            .open_database("project", "project.db")
            .await
            .or_else(|e| Err(crate::Error::from(e)))
    }

    pub fn id(&self) -> String {
        self.config().id
    }
}

#[async_trait::async_trait]
pub trait ProjectsExt {
    async fn projects_directory(&self) -> PathBuf;
    async fn create_project(&self, config: ProjectConfig) -> crate::Result<Project>;
    async fn existing_project(&self, config: ProjectConfig) -> crate::Result<Project>;
    async fn remove_project(&self, id: String) -> crate::Result<()>;
    async fn list_projects(&self) -> crate::Result<Vec<ProjectConfig>>;
}

#[async_trait::async_trait]
impl<T: Manager<tauri::Wry> + Send + Sync> ProjectsExt for T {
    async fn projects_directory(&self) -> PathBuf {
        self.app_state().await.project_path
    }

    async fn create_project(&self, config: ProjectConfig) -> crate::Result<Project> {
        let target = self.projects_directory().await.join(config.id.clone());
        if target.exists() {
            return Err(crate::ProjectError::exists(config.id));
        }

        let persistence = self.app_handle().persistence();
        let ctx = persistence
            .context(ContextSpecifier::Direct {
                alias: config.id.clone(),
                path: target.clone().to_str().unwrap().to_string(),
            })
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;

        let conf_handle = ctx
            .open_file_handle("project.json", FileHandleMode::create_new(true))
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;
        let conf_locked = conf_handle.handle().await;
        let mut config_file = conf_locked.lock();

        config_file
            .write_all(to_string_pretty(&config).unwrap().as_bytes())
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;
        config_file
            .flush()
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;
        conf_handle
            .close()
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;

        Ok(Project {
            context: ctx,
            config: config.clone(),
            handle: self.app_handle().clone(),
        })
    }
    async fn existing_project(&self, config: ProjectConfig) -> crate::Result<Project> {
        let target = self.projects_directory().await.join(config.id.clone());
        if !target.exists() {
            return Err(crate::ProjectError::not_exists(config.id));
        }

        let persistence = self.app_handle().persistence();
        let ctx = persistence
            .context(ContextSpecifier::Direct {
                alias: config.id.clone(),
                path: target.clone().to_str().unwrap().to_string(),
            })
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;

        let conf_handle = ctx
            .open_file_handle("project.json", FileHandleMode::read())
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;
        let conf_locked = conf_handle.handle().await;
        let mut config_file = conf_locked.lock();
        let mut buf: Vec<u8> = Vec::new();

        config_file
            .read_to_end(&mut buf)
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;
        let deserialized = from_slice::<ProjectConfig>(buf.as_slice())
            .or_else(|e| Err(crate::Error::deserialization(e)))?;
        conf_handle
            .close()
            .await
            .or_else(|e| Err(crate::Error::from(e)))?;

        Ok(Project {
            context: ctx,
            config: deserialized,
            handle: self.app_handle().clone(),
        })
    }

    async fn remove_project(&self, id: String) -> crate::Result<()> {
        let target = self.projects_directory().await.join(id.clone());
        if !target.exists() {
            return Err(crate::ProjectError::not_exists(id.clone()));
        }

        fs::remove_dir_all(target).or_else(|e| Err(crate::Error::from(e)))?;
        Ok(())
    }

    async fn list_projects(&self) -> crate::Result<Vec<ProjectConfig>> {
        let mut results: Vec<ProjectConfig> = Vec::new();
        for folder in
            fs::read_dir(self.projects_directory().await).or_else(|e| Err(crate::Error::from(e)))?
        {
            if let Ok(dir) = folder {
                if dir.path().join("project.json").exists() {
                    if let Ok(data) = fs::read(dir.path().join("project.json")) {
                        if let Ok(deserialized) = from_slice::<ProjectConfig>(data.as_slice()) {
                            results.push(deserialized);
                        }
                    }
                }
            }
        }

        Ok(results)
    }
}
