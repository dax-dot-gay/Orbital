use tauri::{ipc::Invoke, Wry};
use taurpc::Router;

mod asset_version;

#[taurpc::procedures(export_to = "../src/bindings.ts")]
trait Api {
    async fn app_version() -> String;
}

#[derive(Clone)]
struct ApiImpl;

#[taurpc::resolvers]
impl Api for ApiImpl {
    async fn app_version(self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}

pub fn routes() -> impl Fn(Invoke) -> bool {
    let mut router = Router::<Wry>::new()
        .merge(ApiImpl.into_handler());

    #[cfg(debug_assertions)]
    {
        router = router.export_config(
            specta_typescript::Typescript::default()
                .header("// Auto-generated bindings. DO NOT MODIFY.")
                .bigint(specta_typescript::BigIntExportBehavior::String)
                .formatter(specta_typescript::formatter::prettier),
        );
    }
    

    router.into_handler()
}
