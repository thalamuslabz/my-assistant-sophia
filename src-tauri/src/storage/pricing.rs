// Token pricing and cost estimation for different providers
// Prices are per 1M tokens (as of January 2026)

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ModelPricing {
    pub input_price_per_1m: f64,  // USD per 1M input tokens
    pub output_price_per_1m: f64, // USD per 1M output tokens
}

pub struct PricingCalculator {
    pricing: HashMap<String, ModelPricing>,
}

impl PricingCalculator {
    pub fn new() -> Self {
        let mut pricing = HashMap::new();

        // Gemini pricing (free tier: 15 RPM, 1M TPM, 1500 RPD)
        pricing.insert("gemini-2.0-flash-exp".to_string(), ModelPricing {
            input_price_per_1m: 0.0,  // Free during preview
            output_price_per_1m: 0.0,
        });
        pricing.insert("gemini-1.5-flash".to_string(), ModelPricing {
            input_price_per_1m: 0.075,
            output_price_per_1m: 0.30,
        });
        pricing.insert("gemini-1.5-pro".to_string(), ModelPricing {
            input_price_per_1m: 1.25,
            output_price_per_1m: 5.00,
        });

        // OpenAI pricing
        pricing.insert("gpt-4o-mini".to_string(), ModelPricing {
            input_price_per_1m: 0.150,
            output_price_per_1m: 0.600,
        });
        pricing.insert("gpt-4o".to_string(), ModelPricing {
            input_price_per_1m: 2.50,
            output_price_per_1m: 10.00,
        });
        pricing.insert("gpt-3.5-turbo".to_string(), ModelPricing {
            input_price_per_1m: 0.50,
            output_price_per_1m: 1.50,
        });

        // Anthropic pricing
        pricing.insert("claude-3-5-haiku-20241022".to_string(), ModelPricing {
            input_price_per_1m: 1.00,
            output_price_per_1m: 5.00,
        });
        pricing.insert("claude-3-5-sonnet-20241022".to_string(), ModelPricing {
            input_price_per_1m: 3.00,
            output_price_per_1m: 15.00,
        });
        pricing.insert("claude-3-opus-20240229".to_string(), ModelPricing {
            input_price_per_1m: 15.00,
            output_price_per_1m: 75.00,
        });

        // DeepSeek pricing (very competitive)
        pricing.insert("deepseek-chat".to_string(), ModelPricing {
            input_price_per_1m: 0.14,
            output_price_per_1m: 0.28,
        });
        pricing.insert("deepseek-coder".to_string(), ModelPricing {
            input_price_per_1m: 0.14,
            output_price_per_1m: 0.28,
        });

        // OpenRouter (varies by model, using common defaults)
        pricing.insert("openai/gpt-4o".to_string(), ModelPricing {
            input_price_per_1m: 2.50,
            output_price_per_1m: 10.00,
        });
        pricing.insert("openai/gpt-3.5-turbo".to_string(), ModelPricing {
            input_price_per_1m: 0.50,
            output_price_per_1m: 1.50,
        });

        // Ollama (local, free)
        pricing.insert("llama3.2:3b".to_string(), ModelPricing {
            input_price_per_1m: 0.0,
            output_price_per_1m: 0.0,
        });

        PricingCalculator { pricing }
    }

    pub fn calculate_cost(&self, model: &str, prompt_tokens: i64, completion_tokens: i64) -> f64 {
        if let Some(pricing) = self.pricing.get(model) {
            let input_cost = (prompt_tokens as f64 / 1_000_000.0) * pricing.input_price_per_1m;
            let output_cost = (completion_tokens as f64 / 1_000_000.0) * pricing.output_price_per_1m;
            input_cost + output_cost
        } else {
            // Unknown model, use conservative estimate
            log::warn!("Unknown model pricing: {}, using default estimate", model);
            let input_cost = (prompt_tokens as f64 / 1_000_000.0) * 1.0;
            let output_cost = (completion_tokens as f64 / 1_000_000.0) * 3.0;
            input_cost + output_cost
        }
    }

    pub fn get_pricing(&self, model: &str) -> Option<&ModelPricing> {
        self.pricing.get(model)
    }
}

// Simple token estimation (rough approximation)
// Real token counting requires tiktoken or similar
pub fn estimate_tokens(text: &str) -> i64 {
    // Rough estimate: 1 token ≈ 4 characters for English text
    // This is a simplification; actual tokenization varies by model
    let chars = text.len() as i64;
    (chars / 4).max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pricing_calculator() {
        let calc = PricingCalculator::new();

        // Test Gemini (free)
        let cost = calc.calculate_cost("gemini-2.0-flash-exp", 1000, 500);
        assert_eq!(cost, 0.0);

        // Test OpenAI gpt-4o-mini
        let cost = calc.calculate_cost("gpt-4o-mini", 1_000_000, 1_000_000);
        assert_eq!(cost, 0.150 + 0.600); // $0.75 per 1M tokens each

        // Test unknown model (uses default)
        let cost = calc.calculate_cost("unknown-model", 1_000_000, 1_000_000);
        assert_eq!(cost, 1.0 + 3.0); // $4.00 default
    }

    #[test]
    fn test_token_estimation() {
        let text = "Hello, world!";
        let tokens = estimate_tokens(text);
        assert!(tokens > 0);
        assert!(tokens <= text.len() as i64);

        let long_text = "a".repeat(1000);
        let tokens = estimate_tokens(&long_text);
        assert_eq!(tokens, 250); // 1000 chars / 4
    }

    #[test]
    fn test_cost_comparison() {
        let calc = PricingCalculator::new();

        let prompt_tokens = 1000;
        let completion_tokens = 500;

        let gemini_cost = calc.calculate_cost("gemini-2.0-flash-exp", prompt_tokens, completion_tokens);
        let openai_cost = calc.calculate_cost("gpt-4o-mini", prompt_tokens, completion_tokens);
        let deepseek_cost = calc.calculate_cost("deepseek-chat", prompt_tokens, completion_tokens);

        // Gemini should be cheapest (free)
        assert!(gemini_cost <= deepseek_cost);
        assert!(deepseek_cost < openai_cost);
    }
}
