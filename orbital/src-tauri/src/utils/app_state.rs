use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;
use bevy_reflect::{GetPath, Reflect};
use derive_builder::Builder;
use rust_patch::Patch;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct AppState {
    pub project_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone, Builder, Patch)]
#[patch = "AppState"]
#[builder(name = "StatePatch", setter(into, strip_option))]
pub struct AppStatePatcher {
    pub project_path: Option<PathBuf>,
}

impl AppState {
    pub fn attach<R: tauri::Runtime>(handle: &AppHandle<R>) {
        let resolver = handle.path();

        handle.manage(Arc::new(Mutex::new(Self {
            project_path: resolver
                .app_local_data_dir()
                .expect("Should be able to retrieve app local data dir."),
        })));
    }
}

#[async_trait]
pub trait AppStateExt<R: tauri::Runtime> {
    async fn app_state(&self) -> AppState;
    async fn app_state_key<V: Send + Sync + Reflect + Clone>(
        &self,
        path: impl AsRef<str> + Send + Sync,
    ) -> Option<V>;
    async fn patch_app_state(&self, patch: StatePatch) -> AppState;
}

#[async_trait]
impl<R: tauri::Runtime, T: Manager<R> + Send + Sync> AppStateExt<R> for T {
    async fn app_state(&self) -> AppState {
        let state = self.state::<Arc<Mutex<AppState>>>();
        let locked = state.lock().await;
        locked.clone()
    }

    async fn patch_app_state(&self, patch: StatePatch) -> AppState {
        let state = self.state::<Arc<Mutex<AppState>>>();
        let mut locked = state.lock().await;
        *locked = patch.build().unwrap().apply(locked.clone());
        locked.clone()
    }

    async fn app_state_key<V: Send + Sync + Reflect + Clone>(
        &self,
        path: impl AsRef<str> + Send + Sync,
    ) -> Option<V> {
        let state = self.app_state().await;
        if let Ok(result) = state.path::<V>(path.as_ref()) {
            Some(result.clone())
        } else {
            None
        }
    }
}
