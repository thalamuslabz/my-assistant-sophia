use crate::runtime::audit::AuditLogger;
use crate::runtime::state::RuntimeState;
use serde_json::json;
use std::sync::Mutex;
use tauri::AppHandle;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct RuntimeManager {
    state: Arc<Mutex<RuntimeState>>,
    logger: Arc<AuditLogger>,
}

impl RuntimeManager {
    pub fn new(app_handle: &AppHandle) -> Self {
        RuntimeManager {
            state: Arc::new(Mutex::new(RuntimeState::Stopped)),
            logger: Arc::new(AuditLogger::from_app(app_handle)),
        }
    }

    pub fn get_state(&self) -> RuntimeState {
        *self.state.lock().unwrap()
    }

    fn spawn_runtime_loop(&self) {
        let state_clone = self.state.clone();
        let logger_clone = self.logger.clone();

        thread::spawn(move || {
            loop {
                let current_state = *state_clone.lock().unwrap();
                
                match current_state {
                    RuntimeState::Running => {
                        // Observe -> Interpret -> Act loop would go here
                        // For now, just sleep to prevent busy loop
                        thread::sleep(Duration::from_millis(100));
                    },
                    RuntimeState::Paused => {
                        // HALT: Do nothing, just wait
                        thread::sleep(Duration::from_millis(100));
                    },
                    RuntimeState::Stopped | RuntimeState::Error => {
                        // Exit loop
                        break;
                    },
                    RuntimeState::Starting => {
                        // Wait for transition to Running
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            }
            
            logger_clone.log("INFO", "runtime", "loop_exit", json!({}));
        });
    }

    pub fn transition_to(&self, target: RuntimeState, reason: &str) -> Result<(), String> {
        let mut current_state = self.state.lock().unwrap();
        
        if current_state.can_transition_to(&target) {
            let previous = *current_state;
            *current_state = target;

            self.logger.log(
                "INFO",
                "runtime",
                "state_transition",
                json!({
                    "from": previous,
                    "to": target,
                    "reason": reason
                }),
            );

            Ok(())
        } else {
            let error_msg = format!("Invalid transition from {:?} to {:?}", *current_state, target);
            
            self.logger.log(
                "ERROR",
                "runtime",
                "invalid_transition_attempt",
                json!({
                    "from": *current_state,
                    "to": target,
                    "reason": reason,
                    "error": error_msg
                }),
            );

            Err(error_msg)
        }
    }

    pub fn start(&self) -> Result<(), String> {
        self.transition_to(RuntimeState::Starting, "System Startup")?;
        
        // Spawn the loop
        self.spawn_runtime_loop();
        
        // Simulate startup checks...
        self.transition_to(RuntimeState::Running, "Startup Complete")
    }

    pub fn stop(&self) -> Result<(), String> {
        self.transition_to(RuntimeState::Stopped, "System Shutdown")
    }

    pub fn pause(&self) -> Result<(), String> {
        self.transition_to(RuntimeState::Paused, "User Requested Pause")
    }

    pub fn resume(&self) -> Result<(), String> {
        self.transition_to(RuntimeState::Running, "User Requested Resume")
    }
}
