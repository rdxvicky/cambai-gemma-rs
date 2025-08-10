mod asr;
mod gemma;
mod platform;
#[cfg(feature = "ui")] mod ui;

#[cfg(feature = "realtime")]
use crate::asr::transcribe_wav;
#[cfg(feature = "realtime")]
use crate::asr::AsrConfig;
#[cfg(feature = "realtime")]
use crate::gemma::{translate, Direction, GemmaConfig};
use clap::{ArgGroup, Parser};
use log::LevelFilter;

#[derive(Parser, Debug)]
#[command(version, about = "Gemma-powered speech translator (Whisper API + Gemma translation)")]
#[command(group(ArgGroup::new("input").required(false).args(["wav", "realtime"])))]
struct Args {
    /// Path to mono 16kHz WAV file
    #[arg(long)]
    wav: Option<String>,

    /// Realtime mic capture (seconds)
    #[arg(long)]
    realtime: Option<u32>,

    /// Direction: es-en or en-es
    #[arg(long, value_parser = ["es-en", "en-es"])]
    direction: String,

    /// OpenAI API key (or set OPENAI_API_KEY env var)
    #[arg(long)]
    api_key: Option<String>,
    
    /// Use local Whisper API instead of OpenAI
    #[arg(long, default_value_t = false)]
    local: bool,

    /// Path to Gemma model (GGUF)
    #[arg(long)]
    gemma_model: String,

    /// Context tokens for Gemma
    #[arg(long, default_value_t = 2048)]
    gemma_ctx: usize,

    /// Run local UI (http://localhost:PORT)
    #[arg(long, default_value_t = false)]
    ui: bool,

    /// UI port
    #[arg(long, default_value_t = 8080)]
    port: u16,

    /// Verbose logs
    #[arg(long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    env_logger::Builder::from_default_env()
        .filter_level(if args.verbose { LevelFilter::Debug } else { LevelFilter::Info })
        .init();

    if args.ui {
        #[cfg(feature = "ui")] {
            let port = args.port;
            println!("UI: http://localhost:{port}");
            actix_web::rt::System::new().block_on(ui::ui::run(port)).unwrap();
            return;
        }
        #[cfg(not(feature = "ui"))]
        {
            eprintln!("Rebuild with --features ui to enable the interface");
            return;
        }
    }

    // Validate input arguments when not in UI mode
    if args.wav.is_none() && args.realtime.is_none() {
        eprintln!("Error: Either --wav or --realtime must be provided when not using --ui mode.");
        eprintln!("Use --help for more information.");
        std::process::exit(1);
    }
    
    #[cfg(feature = "realtime")]
    {
        let dir = Direction::from_str(&args.direction).expect("Invalid direction");
        
        // Get API key from args or environment
        let api_key = args.api_key.or_else(|| std::env::var("OPENAI_API_KEY").ok());
        
        let asr_cfg = AsrConfig { 
            api_key,
            use_local: args.local 
        };
        let text = if let Some(path) = args.wav.as_ref() {
            transcribe_wav(path, &asr_cfg).unwrap_or_else(|e| {
                eprintln!("ASR error: {}", e);
                std::process::exit(1);
            })
        } else {
            let secs = args.realtime.unwrap_or(5);
            asr::realtime::record_and_transcribe(&asr_cfg, secs).unwrap_or_else(|e| {
                eprintln!("Recording error: {}", e);
                std::process::exit(1);
            })
        };

        let gemma_cfg = GemmaConfig { model_path: args.gemma_model.clone(), n_ctx: args.gemma_ctx };
        let translated = translate(&gemma_cfg, dir, &text).unwrap_or_else(|e| {
            eprintln!("Translation error: {}", e);
            std::process::exit(1);
        });
        println!("{}", translated);
    }
    #[cfg(not(feature = "realtime"))]
    {
        eprintln!("This application requires the 'realtime' feature to be enabled.");
        eprintln!("Rebuild with: cargo build --features realtime");
    }
}