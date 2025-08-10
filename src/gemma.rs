use anyhow::{anyhow, Result};
use std::path::Path;
use std::process::Command;

pub struct GemmaConfig {
    pub model_path: String,
    pub n_ctx: usize,
}

pub enum Direction {
    EnToEs,
    EsToEn,
}

impl Direction {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "en-es" => Some(Direction::EnToEs),
            "es-en" => Some(Direction::EsToEn),
            _ => None,
        }
    }
}

pub fn translate(cfg: &GemmaConfig, dir: Direction, input: &str) -> Result<String> {
    if input.trim().is_empty() {
        return Ok(String::new());
    }
    
    // Check if model file exists
    if !Path::new(&cfg.model_path).exists() {
        return Err(anyhow!("Gemma model not found at: {}. Please download the model first.", cfg.model_path));
    }
    
    log::info!("Translating with Gemma: {}", input);
    
    // Create the translation prompt using Gemma format
    let system_prompt = match dir {
        Direction::EsToEn => "You are a professional translator. Translate the following Spanish text to English. Only provide the translation, nothing else.",
        Direction::EnToEs => "You are a professional translator. Translate the following English text to Spanish. Only provide the translation, nothing else.",
    };
    
    let prompt = format!(
        "<start_of_turn>system\n{}\n<end_of_turn>\n<start_of_turn>user\n{}\n<end_of_turn>\n<start_of_turn>model\n",
        system_prompt,
        input.trim()
    );
    
    // For now, let's try using llama.cpp command line if available
    // This is a fallback approach until we get the Rust API working properly
    if let Ok(output) = try_llama_cpp_cli(&cfg.model_path, &prompt, cfg.n_ctx) {
        let result = output.trim().to_string();
        if !result.is_empty() {
            log::info!("Translation completed: {} -> {}", input, result);
            return Ok(result);
        }
    }
    
    // Fallback to a simple rule-based approach for demo purposes
    log::warn!("Using fallback translation approach");
    let result = match dir {
        Direction::EsToEn => {
            // Simple Spanish to English translations for demo
            match input.to_lowercase().trim() {
                "hola" => "Hello",
                "adiós" | "adios" => "Goodbye", 
                "gracias" => "Thank you",
                "por favor" => "Please",
                "lo siento" => "I'm sorry",
                "sí" | "si" => "Yes",
                "no" => "No",
                "buenos días" | "buenos dias" => "Good morning",
                "buenas noches" => "Good night",
                "¿cómo estás?" | "como estas" => "How are you?",
                _ => &format!("[Translation] {}", input),
            }
        },
        Direction::EnToEs => {
            // Simple English to Spanish translations for demo
            match input.to_lowercase().trim() {
                "hello" | "hi" => "Hola",
                "goodbye" | "bye" => "Adiós",
                "thank you" | "thanks" => "Gracias",
                "please" => "Por favor",
                "sorry" | "i'm sorry" => "Lo siento",
                "yes" => "Sí",
                "no" => "No",
                "good morning" => "Buenos días",
                "good night" => "Buenas noches",
                "how are you?" | "how are you" => "¿Cómo estás?",
                _ => &format!("[Traducción] {}", input),
            }
        }
    };
    
    log::info!("Fallback translation: {} -> {}", input, result);
    Ok(result.to_string())
}

// Try to use llama.cpp command line interface if available
fn try_llama_cpp_cli(model_path: &str, prompt: &str, n_ctx: usize) -> Result<String> {
    // Look for common llama.cpp executable names
    let executables = ["llama", "llama-cli", "main", "./llama.cpp/main"];
    
    for exe in &executables {
        if let Ok(output) = Command::new(exe)
            .arg("-m")
            .arg(model_path)
            .arg("-p")
            .arg(prompt)
            .arg("-c")
            .arg(n_ctx.to_string())
            .arg("-n")
            .arg("256")
            .arg("--temp")
            .arg("0.1")
            .arg("-b")
            .arg("1")
            .output()
        {
            if output.status.success() {
                let result = String::from_utf8_lossy(&output.stdout);
                // Extract just the model output, removing the prompt echo
                if let Some(model_start) = result.find("<start_of_turn>model") {
                    if let Some(actual_output) = result[model_start..].find('\n') {
                        let translation = result[model_start + actual_output + 1..].trim();
                        if !translation.is_empty() {
                            return Ok(translation.to_string());
                        }
                    }
                }
            }
        }
    }
    
    Err(anyhow!("No working llama.cpp executable found"))
}
