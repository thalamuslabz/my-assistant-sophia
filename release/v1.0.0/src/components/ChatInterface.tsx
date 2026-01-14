import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export function ChatInterface() {
  const [input, setInput] = useState("");
  const [history, setHistory] = useState<{role: string, content: string}[]>([]);
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!input.trim() || loading) return;

    const prompt = input;
    setInput("");
    setLoading(true);
    setHistory(prev => [...prev, { role: "user", content: prompt }]);

    try {
      const response = await invoke<string>("submit_prompt", { prompt });
      setHistory(prev => [...prev, { role: "assistant", content: response }]);
    } catch (err) {
      setHistory(prev => [...prev, { role: "error", content: String(err) }]);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="chat-interface">
      <div className="chat-history">
        {history.map((msg, i) => (
          <div key={i} className={`msg msg-${msg.role}`}>
            <strong>{msg.role}:</strong> {msg.content}
          </div>
        ))}
        {loading && <div className="msg msg-system">Thinking...</div>}
      </div>
      <form onSubmit={handleSubmit}>
        <input 
          value={input}
          onChange={e => setInput(e.target.value)}
          placeholder="Type a command..."
          disabled={loading}
        />
        <button type="submit" disabled={loading}>Send</button>
      </form>
    </div>
  );
}
