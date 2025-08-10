#[cfg(feature = "ui")]
pub mod ui {
    use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
    use serde::{Deserialize, Serialize};
    use sysinfo::System;

    #[derive(Serialize)]
    struct Stats { cpu_usage: f32, mem_used_mb: u64, mem_total_mb: u64 }

    #[get("/stats")]
    async fn stats() -> impl Responder {
        let mut sys = System::new_all();
        sys.refresh_all();
        let cpu_usage = sys.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>() / (sys.cpus().len() as f32).max(1.0);
        let mem_used_mb = sys.used_memory() / (1024 * 1024);
        let mem_total_mb = sys.total_memory() / (1024 * 1024);
        web::Json(Stats { cpu_usage, mem_used_mb, mem_total_mb })
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
        HttpServer::new(|| App::new().service(index).service(styles).service(stats).service(translate))
            .bind(("0.0.0.0", port))?
            .run()
            .await
    }
}