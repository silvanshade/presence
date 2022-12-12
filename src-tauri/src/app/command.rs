pub mod api {
    pub mod twitch {
        #[tauri::command]
        pub async fn authorization_flow<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
            crate::api::twitch::authorization_flow(&app)
                .await
                .map_err(|err| err.to_string())
        }
    }
}
