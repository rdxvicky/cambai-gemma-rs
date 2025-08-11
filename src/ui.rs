#[cfg(feature = "ui")]
pub mod ui {
    use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
    use serde::{Deserialize, Serialize};
    use sysinfo::{System, ProcessRefreshKind, RefreshKind, MemoryRefreshKind};
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, SystemTime};
    use std::process;
    use lazy_static::lazy_static;

    #[derive(Serialize, Clone)]
    struct PerfData {
        timestamp: u64,
        cpu_usage: f32,
        mem_used_mb: u64,
    }

    #[derive(Serialize)]
    struct Stats { 
        cpu_usage: f32, 
        mem_used_mb: u64, 
        mem_total_mb: u64,
        peak_cpu: f32,
        peak_mem_mb: u64,
        performance_history: Vec<PerfData>,
    }

    #[derive(Default)]
    struct PerfTracker {
        peak_cpu: f32,
        peak_mem: u64,
        history: Vec<PerfData>,
        max_history: usize,
    }

    lazy_static! {
        static ref PERFORMANCE_TRACKER: Arc<Mutex<PerfTracker>> = Arc::new(Mutex::new(
            PerfTracker {
                peak_cpu: 0.0,
                peak_mem: 0,
                history: Vec::new(),
                max_history: 60, // Keep last 60 data points (5 minutes at 5-second intervals)
            }
        ));
        static ref SYSTEM: Arc<Mutex<System>> = Arc::new(Mutex::new(
            System::new_with_specifics(
                RefreshKind::new()
                    .with_processes(ProcessRefreshKind::new().with_memory().with_cpu())
                    .with_memory(MemoryRefreshKind::default())
            )
        ));
    }

    #[get("/stats")]
    async fn stats() -> impl Responder {
        // Use the persistent system instance for better CPU tracking
        let (app_cpu_usage, app_mem_mb, total_mem_mb) = {
            let mut sys = SYSTEM.lock().unwrap();
            
            // Refresh all system information for accurate readings
            sys.refresh_all();
            
            // Get current process ID
            let current_pid = process::id();
            let pid = sysinfo::Pid::from_u32(current_pid);
            
            // Get application-specific metrics
            let (cpu_usage, mem_mb) = if let Some(process) = sys.process(pid) {
                let cpu = process.cpu_usage();
                let memory_bytes = process.memory();
                let memory_mb = memory_bytes / (1024 * 1024);
                (cpu, memory_mb)
            } else {
                (0.0, 0)
            };
            
            // Get total system memory
            let total_mem_mb = sys.total_memory() / (1024 * 1024);
            
            (cpu_usage, mem_mb, total_mem_mb)
        };
        
        // Update performance tracking
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
            
        let perf_data = PerfData {
            timestamp,
            cpu_usage: app_cpu_usage,
            mem_used_mb: app_mem_mb,
        };
        
        let (peak_cpu, peak_mem, history) = {
            let mut tracker = PERFORMANCE_TRACKER.lock().unwrap();
            
            // Update peaks
            if app_cpu_usage > tracker.peak_cpu {
                tracker.peak_cpu = app_cpu_usage;
            }
            if app_mem_mb > tracker.peak_mem {
                tracker.peak_mem = app_mem_mb;
            }
            
            // Add to history
            tracker.history.push(perf_data.clone());
            
            // Keep only recent history
            if tracker.history.len() > tracker.max_history {
                tracker.history.remove(0);
            }
            
            (tracker.peak_cpu, tracker.peak_mem, tracker.history.clone())
        };
        
        web::Json(Stats { 
            cpu_usage: app_cpu_usage, 
            mem_used_mb: app_mem_mb, 
            mem_total_mb: total_mem_mb,
            peak_cpu,
            peak_mem_mb: peak_mem,
            performance_history: history,
        })
    }

    #[post("/reset-stats")]
    async fn reset_stats() -> impl Responder {
        let mut tracker = PERFORMANCE_TRACKER.lock().unwrap();
        tracker.peak_cpu = 0.0;
        tracker.peak_mem = 0;
        tracker.history.clear();
        
        HttpResponse::Ok().json(serde_json::json!({
            "ok": true,
            "message": "Performance statistics reset successfully"
        }))
    }

    #[derive(Deserialize)]
    pub struct JobReq { direction: String, text: String }

    #[post("/translate")]
    async fn translate(req: web::Json<JobReq>) -> impl Responder {
        use crate::gemma::{translate as gemma_translate, Direction, GemmaConfig};
        
        // Parse direction
        let direction = match Direction::from_str(&req.direction) {
            Some(dir) => dir,
            None => {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "ok": false,
                    "error": "Invalid direction. Use 'es-en' or 'en-es'"
                }));
            }
        };
        
        // Use hardcoded model path for UI (you can make this configurable later)
        let gemma_cfg = GemmaConfig {
            model_path: "models/gemma-2b-it.Q4_K_M.gguf".to_string(),
            n_ctx: 2048,
        };
        
        // Perform translation
        match gemma_translate(&gemma_cfg, direction, &req.text) {
            Ok(translated_text) => {
                HttpResponse::Ok().json(serde_json::json!({
                    "ok": true,
                    "direction": req.direction,
                    "original": req.text,
                    "translated": translated_text
                }))
            }
            Err(e) => {
                log::error!("Translation failed: {}", e);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "ok": false,
                    "error": format!("Translation failed: {}", e)
                }))
            }
        }
    }

    #[get("/")]
    async fn index() -> impl Responder {
        let html = include_str!("../static/index.html");
        HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)
    }

    #[get("/styles.css")]
    async fn styles() -> impl Responder {
        let css = include_str!("../static/styles.css");
        HttpResponse::Ok().content_type("text/css; charset=utf-8").body(css)
    }

    pub async fn run(port: u16) -> std::io::Result<()> {
        // Initialize the system for better CPU tracking
        {
            let mut sys = SYSTEM.lock().unwrap();
            sys.refresh_all();
            // Wait a bit to allow initial CPU measurement
            std::thread::sleep(Duration::from_millis(500));
            sys.refresh_all();
        }
        
        HttpServer::new(|| App::new().service(index).service(styles).service(stats).service(reset_stats).service(translate))
            .bind(("0.0.0.0", port))?
            .run()
            .await
    }
}