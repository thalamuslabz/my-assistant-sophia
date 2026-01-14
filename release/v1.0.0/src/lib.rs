pub mod runtime;
pub mod storage;
pub mod onboarding;
pub mod router;

use runtime::{RuntimeManager, RuntimeState};
use storage::StorageManager;
use onboarding::OnboardingManager;
use router::ModelRouter;
use std::sync::Arc;
use tauri::{State, Manager};

// --- Commands ---

#[tauri::command]
fn get_runtime_state(state: State<RuntimeManager>) -> RuntimeState {
    state.get_state()
}

#[tauri::command]
fn pause_runtime(state: State<RuntimeManager>) -> Result<(), String> {
    state.pause()
}

#[tauri::command]
fn resume_runtime(state: State<RuntimeManager>) -> Result<(), String> {
    state.resume()
}

#[tauri::command]
fn start_runtime(state: State<RuntimeManager>) -> Result<(), String> {
    state.start()
}

#[tauri::command]
fn check_onboarding_status(state: State<OnboardingManager>) -> bool {
    state.has_completed_onboarding()
}

#[tauri::command]
fn complete_onboarding(state: State<OnboardingManager>, contract_version: String, contract_hash: String) -> Result<(), String> {
    state.accept_contract(&contract_version, &contract_hash)
}

#[tauri::command]
async fn submit_prompt(
    prompt: String,
    router: State<'_, ModelRouter>,
    runtime: State<'_, RuntimeManager>
) -> Result<String, String> {
    // 1. Guardrail: Check Pause State
    if runtime.get_state() == RuntimeState::Paused {
        return Err("Runtime is PAUSED. Request rejected.".to_string());
    }

    // 2. Route & Execute (Blocking call wrapped in async)
    // In a real app, use tokio::spawn_blocking
    router.route_and_execute(&prompt)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize Managers
            let runtime_manager = RuntimeManager::new(app.handle());
            let storage_manager = Arc::new(StorageManager::new(app.handle()));
            let onboarding_manager = OnboardingManager::new(storage_manager.clone());
            let model_router = ModelRouter::new(storage_manager.clone());

            // Manage State
            app.manage(runtime_manager);
            app.manage(storage_manager); 
            app.manage(onboarding_manager);
            app.manage(model_router);
            
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_runtime_state,
            pause_runtime,
            resume_runtime,
            start_runtime,
            check_onboarding_status,
            complete_onboarding,
            submit_prompt
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
