pub mod runtime;
pub mod storage;
pub mod onboarding;
pub mod router;
pub mod secret_store;
pub mod providers;

use runtime::{RuntimeManager, RuntimeState};
use storage::StorageManager;
use onboarding::OnboardingManager;
use router::ModelRouter;
use secret_store::SecretStore;
use providers::{ProviderRegistry, ProviderType, ProviderConfig};
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
fn complete_onboarding(
    state: State<OnboardingManager>,
    provider_registry: State<'_, Arc<std::sync::Mutex<ProviderRegistry>>>,
    storage: State<'_, Arc<StorageManager>>,
    contract_version: String,
    contract_hash: String,
    gemini_key_id: String,
    gemini_key_value: String,
    network_egress_consent: bool,
) -> Result<(), String> {
    log::info!("Starting onboarding completion with key_id: {}", gemini_key_id);

    let mut registry = provider_registry.lock().map_err(|_| "Registry lock error".to_string())?;
    
    log::info!("Setting API key in keychain...");
    registry.set_api_key(&gemini_key_id, &gemini_key_value)?;
    log::info!("API key stored successfully");
    
    registry.set_provider_enabled(&ProviderType::Gemini, true);
    log::info!("Gemini provider enabled");

    if let Some(config) = registry.get_provider_config(&ProviderType::Gemini) {
        log::info!("Saving provider config with keychain_id: {}", config.api_key_keychain_id);
        storage.set_preference(&ProviderType::Gemini.preference_key(), serde_json::to_value(config).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    }

    state.accept_contract(
        &contract_version,
        &contract_hash,
        &gemini_key_id,
        network_egress_consent,
    )
}

#[tauri::command]
fn save_provider_key(
    provider_registry: State<'_, Arc<std::sync::Mutex<ProviderRegistry>>>,
    storage: State<'_, Arc<StorageManager>>,
    provider: String,
    api_key: String
) -> Result<(), String> {
    let provider_type = ProviderType::from_str(&provider).ok_or("Unknown provider")?;
    let mut registry = provider_registry.lock().map_err(|_| "Registry lock error".to_string())?;
    
    // Get the keychain ID before modifying
    let keychain_id = registry.get_provider_config(&provider_type)
        .ok_or("Provider not configured")?
        .api_key_keychain_id.clone();

    // Set the key and enable the provider
    registry.set_api_key(&keychain_id, &api_key)?;
    registry.set_provider_enabled(&provider_type, true);
    
    // Save the updated config to storage
    if let Some(config) = registry.get_provider_config(&provider_type) {
        storage.set_preference(&provider_type.preference_key(), serde_json::to_value(config).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
fn update_provider_model(
    provider_registry: State<'_, Arc<std::sync::Mutex<ProviderRegistry>>>,
    storage: State<'_, Arc<StorageManager>>,
    provider: String,
    model: String
) -> Result<(), String> {
    let provider_type = ProviderType::from_str(&provider).ok_or("Unknown provider")?;
    let mut registry = provider_registry.lock().map_err(|_| "Registry lock error".to_string())?;

    if let Some(config) = registry.get_provider_config_mut(&provider_type) {
        config.model = model.clone();
        storage.set_preference(&provider_type.preference_key(), serde_json::to_value(config).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
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

    log::info!("Submitting prompt: {}", &prompt[..prompt.len().min(50)]);
    
    // 2. Route & Execute (Blocking call wrapped in async)
    // In a real app, use tokio::spawn_blocking
    router.route_and_execute(&prompt)
}

#[tauri::command]
fn test_keychain(
    provider_registry: State<'_, Arc<std::sync::Mutex<ProviderRegistry>>>,
) -> Result<String, String> {
    let registry = provider_registry.lock().map_err(|_| "Registry lock error".to_string())?;
    
    // First try to set a test key
    log::info!("Testing keychain write...");
    match registry.set_api_key("test_key", "test_value") {
        Ok(_) => log::info!("Test key written successfully"),
        Err(e) => return Err(format!("Failed to write test key: {}", e))
    }
    
    // Try to read it back
    log::info!("Testing keychain read...");
    match registry.get_api_key("test_key") {
        Ok(Some(val)) => log::info!("Test key read successfully: {}", val),
        Ok(None) => return Err("Test key not found after writing".to_string()),
        Err(e) => return Err(format!("Failed to read test key: {}", e))
    }
    
    // Now check for the actual Gemini key
    log::info!("Checking for Gemini key...");
    match registry.get_api_key("gemini_api_key") {
        Ok(Some(key)) => Ok(format!("Gemini key found: {}...", &key[..key.len().min(10)])),
        Ok(None) => Ok("Gemini key not found in keychain (but test key works)".to_string()),
        Err(e) => Err(format!("Keychain error reading Gemini key: {}", e))
    }
}

#[tauri::command]
fn reset_provider_config(
    provider_registry: State<'_, Arc<std::sync::Mutex<ProviderRegistry>>>,
    storage: State<'_, Arc<StorageManager>>,
    provider: String,
) -> Result<String, String> {
    let provider_type = ProviderType::from_str(&provider).ok_or("Unknown provider")?;
    let mut registry = provider_registry.lock().map_err(|_| "Registry lock error".to_string())?;
    
    // Reset to default config
    let default_config = match provider_type {
        ProviderType::Gemini => ProviderConfig::default_gemini(),
        ProviderType::OpenAI => ProviderConfig::default_openai(),
        ProviderType::Anthropic => ProviderConfig::default_anthropic(),
        ProviderType::DeepSeek => ProviderConfig::default_deepseek(),
        ProviderType::OpenRouter => ProviderConfig::default_openrouter(),
        ProviderType::Ollama => ProviderConfig::default_ollama(),
    };
    
    log::info!("Resetting {} to default config: model={}, endpoint={}", 
        provider, default_config.model, default_config.endpoint);
    
    registry.set_provider_config(default_config.clone());
    storage.set_preference(&provider_type.preference_key(), serde_json::to_value(&default_config).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    
    Ok(format!("Reset {} to model: {}", provider, default_config.model))
}

#[tauri::command]
fn get_usage_stats(
    storage: State<'_, Arc<StorageManager>>,
    days: i64,
) -> Result<Vec<storage::UsageStats>, String> {
    storage.get_all_usage_stats(days)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_total_cost(
    storage: State<'_, Arc<StorageManager>>,
    days: i64,
) -> Result<f64, String> {
    storage.get_total_cost(days)
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize Managers
            let runtime_manager = RuntimeManager::new(app.handle());
            let storage_manager = Arc::new(StorageManager::new(app.handle()));
            let onboarding_manager = OnboardingManager::new(storage_manager.clone());
            let secret_store = Arc::new(SecretStore::new("sophia"));
            let provider_registry = Arc::new(std::sync::Mutex::new(ProviderRegistry::new(secret_store)));

            // Load provider config preferences if present
            for provider in [
                ProviderType::Gemini,
                ProviderType::DeepSeek,
                ProviderType::OpenAI,
                ProviderType::Anthropic,
                ProviderType::OpenRouter,
                ProviderType::Ollama,
            ] {
                if let Ok(Some(val)) = storage_manager.get_preference(&provider.preference_key()) {
                    if let Ok(config) = serde_json::from_value(val) {
                        provider_registry.lock().unwrap().load_provider_config(config);
                    }
                }
            }

            let model_router = ModelRouter::new(storage_manager.clone(), provider_registry.clone());

            // Manage State
            app.manage(runtime_manager);
            app.manage(storage_manager.clone()); 
            app.manage(onboarding_manager);
            app.manage(provider_registry.clone());
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
            save_provider_key,
            update_provider_model,
            submit_prompt,
            test_keychain,
            reset_provider_config,
            get_usage_stats,
            get_total_cost
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
