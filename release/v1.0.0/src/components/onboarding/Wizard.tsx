import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface WizardProps {
  onComplete: () => void;
}

export function OnboardingWizard({ onComplete }: WizardProps) {
  const [step, setStep] = useState(0);

  const steps = [
    <WelcomeStep onNext={() => setStep(1)} />,
    <PrivacyStep onNext={() => setStep(2)} />,
    <ContractStep onComplete={onComplete} />
  ];

  return (
    <div className="onboarding-wizard">
      {steps[step]}
    </div>
  );
}

function WelcomeStep({ onNext }: { onNext: () => void }) {
  return (
    <div className="step">
      <h2>Welcome to Sophia</h2>
      <p>Your local-only, explicitly trusted assistant.</p>
      <button onClick={onNext}>Get Started</button>
    </div>
  );
}

function PrivacyStep({ onNext }: { onNext: () => void }) {
  return (
    <div className="step">
      <h2>Privacy First</h2>
      <ul>
        <li>Everything runs on your device.</li>
        <li>No data is sent to the cloud without approval.</li>
        <li>You hold the keys.</li>
      </ul>
      <button onClick={onNext}>I Understand</button>
    </div>
  );
}

function ContractStep({ onComplete }: { onComplete: () => void }) {
  const [accepted, setAccepted] = useState(false);

  const handleSign = async () => {
    try {
      await invoke('complete_onboarding', {
        contractVersion: 'v1.0',
        contractHash: 'sha256:simulated_hash_of_terms'
      });
      onComplete();
    } catch (e) {
      console.error('Failed to sign contract:', e);
    }
  };

  return (
    <div className="step">
      <h2>Operating Contract</h2>
      <div className="contract-box">
        <p>1. Pause means Pause. Absolutely.</p>
        <p>2. No hidden training on your data.</p>
        <p>3. Full auditability of all actions.</p>
      </div>
      <label>
        <input 
          type="checkbox" 
          checked={accepted} 
          onChange={(e) => setAccepted(e.target.checked)} 
        />
        I agree to these terms explicitly.
      </label>
      <button disabled={!accepted} onClick={handleSign}>
        Sign & Initialize
      </button>
    </div>
  );
}
