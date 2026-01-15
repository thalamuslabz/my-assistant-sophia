import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export function Settings() {
  const [provider, setProvider] = useState("Gemini");
  const [key, setKey] = useState("");
  const [model, setModel] = useState("");
  const [message, setMessage] = useState("");

  const saveKey = async () => {
    try {
      await invoke("save_provider_key", { provider, apiKey: key });
      setMessage("Key saved successfully.");
    } catch (err) {
      setMessage(String(err));
    }
  };

  const updateModel = async () => {
    try {
      await invoke("update_provider_model", { provider, model });
      setMessage("Model updated.");
    } catch (err) {
      setMessage(String(err));
    }
  };

  const testKeychain = async () => {
    try {
      const result = await invoke<string>("test_keychain");
      setMessage(`Keychain test: ${result}`);
    } catch (err) {
      setMessage(`Keychain test failed: ${String(err)}`);
    }
  };

  const resetConfig = async () => {
    try {
      const result = await invoke<string>("reset_provider_config", { provider });
      setMessage(result);
    } catch (err) {
      setMessage(`Reset failed: ${String(err)}`);
    }
  };

  return (
    <div className="settings-panel">
      <h2>Provider Settings</h2>
      <select value={provider} onChange={(e) => setProvider(e.target.value)}>
        <option value="Gemini">Gemini (Default)</option>
        <option value="OpenAI">OpenAI</option>
        <option value="Anthropic">Anthropic</option>
        <option value="DeepSeek">DeepSeek</option>
        <option value="OpenRouter">OpenRouter</option>
      </select>

      <input
        type="password"
        value={key}
        onChange={(e) => setKey(e.target.value)}
        placeholder="Enter API Key"
      />

      <button onClick={saveKey}>Save Key</button>

      <input
        type="text"
        value={model}
        onChange={(e) => setModel(e.target.value)}
        placeholder="Model override (optional)"
      />

      <button onClick={updateModel}>Update Model</button>

      <button onClick={resetConfig}>Reset to Default Config</button>

      <button onClick={testKeychain}>Test Keychain</button>

      {message && <div className="settings-msg">{message}</div>}
    </div>
  );
}
