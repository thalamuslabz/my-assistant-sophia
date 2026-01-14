import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { OnboardingWizard } from "./components/onboarding/Wizard";
import { RuntimeControl } from "./components/RuntimeControl";
import { ChatInterface } from "./components/ChatInterface";

function App() {
  const [isOnboarded, setIsOnboarded] = useState<boolean | null>(null);

  useEffect(() => {
    checkStatus();
  }, []);

  async function checkStatus() {
    try {
      const status = await invoke<boolean>("check_onboarding_status");
      setIsOnboarded(status);
      
      if (status) {
        await invoke("start_runtime").catch(() => {}); // Ignore if already started
      }
    } catch (e) {
      console.error(e);
    }
  }

  if (isOnboarded === null) return <div className="loading">Loading System...</div>;

  if (!isOnboarded) {
    return <OnboardingWizard onComplete={() => checkStatus()} />;
  }

  return (
    <div className="app-container">
      <header className="app-header">
        <h1>Sophia Assistant</h1>
        <RuntimeControl />
      </header>
      
      <main className="app-main">
        <ChatInterface />
      </main>
    </div>
  );
}

export default App;
