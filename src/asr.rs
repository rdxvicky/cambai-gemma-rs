#[cfg(feature = "realtime")]
use anyhow::{anyhow, Result};
#[cfg(feature = "realtime")]
use hound::WavReader;
#[cfg(feature = "realtime")]
use reqwest;
#[cfg(feature = "realtime")]
use serde::{Deserialize, Serialize};

pub struct AsrConfig {
    pub api_key: Option<String>,
    pub use_local: bool,  // If true, use local API, otherwise use OpenAI
}

#[cfg(feature = "realtime")]
#[derive(Serialize, Deserialize, Debug)]
struct WhisperResponse {
    text: String,
}

#[cfg(feature = "realtime")]
pub fn transcribe_wav(path: &str, cfg: &AsrConfig) -> Result<String> {
    // Use async runtime for the API call
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(transcribe_wav_async(path, cfg))
}

#[cfg(feature = "realtime")]
async fn transcribe_wav_async(path: &str, cfg: &AsrConfig) -> Result<String> {
    let reader = WavReader::open(path)?;
    let spec = reader.spec();
    
    log::info!("Processing WAV file: {} channels, {} Hz", spec.channels, spec.sample_rate);
    
    // Read the entire file as bytes for upload
    let file_bytes = std::fs::read(path)
        .map_err(|e| anyhow!("Failed to read audio file: {}", e))?;
    
    // Call Whisper API
    let text = if cfg.use_local {
        call_local_whisper_api(&file_bytes).await?
    } else {
        call_openai_whisper_api(&file_bytes, &cfg.api_key).await?
    };
    
    if text.trim().is_empty() {
        return Err(anyhow!("No speech detected in audio file"));
    }
    
    log::info!("Transcription result: '{}'", text);
    Ok(text)
}

#[cfg(feature = "realtime")]
async fn call_openai_whisper_api(audio_bytes: &[u8], api_key: &Option<String>) -> Result<String> {
    let api_key = api_key.as_ref()
        .ok_or_else(|| anyhow!("OpenAI API key required. Set OPENAI_API_KEY environment variable or pass --api-key"))?;
    
    let client = reqwest::Client::new();
    
    let form = reqwest::multipart::Form::new()
        .text("model", "whisper-1")
        .text("response_format", "json")
        .part(
            "file", 
            reqwest::multipart::Part::bytes(audio_bytes.to_vec())
                .file_name("audio.wav")
                .mime_str("audio/wav")?
        );
    
    let response = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form)
        .send()
        .await
        .map_err(|e| anyhow!("Failed to send request to OpenAI: {}", e))?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(anyhow!("OpenAI API error: {}", error_text));
    }
    
    let whisper_response: WhisperResponse = response
        .json()
        .await
        .map_err(|e| anyhow!("Failed to parse OpenAI response: {}", e))?;
    
    Ok(whisper_response.text)
}

#[cfg(feature = "realtime")]
async fn call_local_whisper_api(audio_bytes: &[u8]) -> Result<String> {
    // Try common local Whisper API endpoints
    let endpoints = [
        "http://localhost:8000/transcribe",
        "http://localhost:5000/transcribe",
        "http://127.0.0.1:8000/transcribe",
    ];
    
    let client = reqwest::Client::new();
    
    for endpoint in &endpoints {
        log::info!("Trying local Whisper API at: {}", endpoint);
        
        let form = reqwest::multipart::Form::new()
            .part(
                "file", 
                reqwest::multipart::Part::bytes(audio_bytes.to_vec())
                    .file_name("audio.wav")
                    .mime_str("audio/wav").unwrap_or_else(|_| {
                        reqwest::multipart::Part::bytes(audio_bytes.to_vec())
                            .file_name("audio.wav")
                    })
            );
        
        match client
            .post(*endpoint)
            .multipart(form)
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => {
                match response.json::<WhisperResponse>().await {
                    Ok(whisper_response) => {
                        log::info!("Successfully used local API at: {}", endpoint);
                        return Ok(whisper_response.text);
                    }
                    Err(e) => {
                        log::warn!("Failed to parse response from {}: {}", endpoint, e);
                        continue;
                    }
                }
            }
            Ok(response) => {
                log::warn!("Local API at {} returned status: {}", endpoint, response.status());
                continue;
            }
            Err(e) => {
                log::debug!("Failed to connect to {}: {}", endpoint, e);
                continue;
            }
        }
    }
    
    Err(anyhow!("No local Whisper API found. Tried: {:?}\nTo use local API, start a Whisper server on one of these endpoints.", endpoints))
}

#[cfg(feature = "realtime")]
pub mod realtime {
    use super::*;
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use std::sync::{Arc, Mutex};

    pub fn record_and_transcribe(cfg: &AsrConfig, seconds: u32) -> Result<String> {
        let host = cpal::default_host();
        let device = host.default_input_device().ok_or(anyhow!("no input device"))?;
        let mut supported = device.supported_input_configs().map_err(|e| anyhow!(e))?;
        // pick 16k mono
        let fmt = supported
            .find(|c| c.min_sample_rate().0 <= 16000 && c.max_sample_rate().0 >= 16000 && c.channels() == 1)
            .ok_or(anyhow!("no mono 16k config"))?
            .with_sample_rate(cpal::SampleRate(16000));
        let config: cpal::StreamConfig = fmt.into();

        let buf = Arc::new(Mutex::new(Vec::<f32>::new()));
        let b2 = buf.clone();
        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _| {
                let mut g = b2.lock().unwrap();
                g.extend_from_slice(data);
            },
            move |err| eprintln!("stream error: {err}"),
            None,
        )?;
        
        println!("Recording for {} seconds...", seconds);
        stream.play()?;
        std::thread::sleep(std::time::Duration::from_secs(seconds as u64));
        drop(stream);
        let samples = buf.lock().unwrap().clone();
        println!("Recording complete. Processing...");

        // Convert f32 samples to i16 and save as temporary WAV file
        let samples_i16: Vec<i16> = samples
            .iter()
            .map(|&sample| (sample.clamp(-1.0, 1.0) * 32767.0) as i16)
            .collect();

        // Create temporary WAV file
        let temp_file = std::env::temp_dir().join("recorded_audio.wav");
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 16000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        
        {
            let mut writer = hound::WavWriter::create(&temp_file, spec)
                .map_err(|e| anyhow!("Failed to create temp WAV file: {}", e))?;
            
            for sample in samples_i16 {
                writer.write_sample(sample)
                    .map_err(|e| anyhow!("Failed to write sample: {}", e))?;
            }
            
            writer.finalize()
                .map_err(|e| anyhow!("Failed to finalize WAV file: {}", e))?;
        }
        
        // Transcribe the temporary WAV file using the API
        let result = transcribe_wav(temp_file.to_str().unwrap(), cfg);
        
        // Clean up temporary file
        let _ = std::fs::remove_file(temp_file);
        
        result
    }
}