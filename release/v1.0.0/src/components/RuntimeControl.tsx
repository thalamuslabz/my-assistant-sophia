import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export function RuntimeControl() {
  const [state, setState] = useState("Unknown");

  const updateState = async () => {
    try {
      const s = await invoke<string>("get_runtime_state");
      setState(s);
    } catch (e) {
      console.error(e);
    }
  };

  useEffect(() => {
    updateState();
    const interval = setInterval(updateState, 1000);
    return () => clearInterval(interval);
  }, []);

  const togglePause = async () => {
    if (state === "Running") {
      await invoke("pause_runtime");
    } else {
      await invoke("resume_runtime");
    }
    updateState();
  };

  return (
    <div className="runtime-control">
      <div className={`status-badge status-${state.toLowerCase()}`}>
        {state}
      </div>
      <button 
        className={`pause-btn ${state === "Running" ? "btn-stop" : "btn-start"}`}
        onClick={togglePause}
      >
        {state === "Running" ? "PAUSE SYSTEM" : "RESUME"}
      </button>
    </div>
  );
}
