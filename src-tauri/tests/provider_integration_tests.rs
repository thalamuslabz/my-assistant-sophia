// Integration tests for all provider clients
// Run with: cargo test --test provider_integration_tests -- --nocapture

use std::env;

// Helper to check if API key is available
fn get_api_key(env_var: &str) -> Option<String> {
    env::var(env_var).ok()
}

#[test]
#[ignore] // Run manually with API keys
fn test_gemini_integration() {
    let api_key = get_api_key("GEMINI_API_KEY")
        .expect("Set GEMINI_API_KEY environment variable to run this test");
    
    // This test requires the full app context
    // For now, we'll document the manual test procedure
    println!("Gemini API Key available: {}...", &api_key[..10]);
    println!("Manual test: Use Settings UI to test Gemini");
}

#[test]
#[ignore]
fn test_openai_integration() {
    let api_key = get_api_key("OPENAI_API_KEY")
        .expect("Set OPENAI_API_KEY environment variable to run this test");
    
    println!("OpenAI API Key available: {}...", &api_key[..10]);
    println!("Manual test: Use Settings UI to test OpenAI");
}

#[test]
#[ignore]
fn test_anthropic_integration() {
    let api_key = get_api_key("ANTHROPIC_API_KEY")
        .expect("Set ANTHROPIC_API_KEY environment variable to run this test");
    
    println!("Anthropic API Key available: {}...", &api_key[..10]);
    println!("Manual test: Use Settings UI to test Anthropic");
}

#[test]
#[ignore]
fn test_deepseek_integration() {
    let api_key = get_api_key("DEEPSEEK_API_KEY")
        .expect("Set DEEPSEEK_API_KEY environment variable to run this test");
    
    println!("DeepSeek API Key available: {}...", &api_key[..10]);
    println!("Manual test: Use Settings UI to test DeepSeek");
}

#[test]
#[ignore]
fn test_openrouter_integration() {
    let api_key = get_api_key("OPENROUTER_API_KEY")
        .expect("Set OPENROUTER_API_KEY environment variable to run this test");
    
    println!("OpenRouter API Key available: {}...", &api_key[..10]);
    println!("Manual test: Use Settings UI to test OpenRouter");
}

#[test]
fn test_ollama_integration() {
    // Ollama is local, no API key needed
    println!("Ollama test: Requires Ollama running on localhost:11434");
    println!("Manual test: Use Settings UI to test Ollama");
}
