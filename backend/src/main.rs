use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::models::JobStatusType;
use crate::handlers::{
    upload::upload_files,
    preview::get_preview,
    create_timelapse::create_timelapse_handler,
    job_status::get_job_status,
    download::download_video,
    health::health_check,
};

mod models;
mod handlers;
mod storage;
mod video;

type JobStore = Arc<Mutex<HashMap<String, JobStatusType>>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize job store
    let job_store: JobStore = Arc::new(Mutex::new(HashMap::new()));
    
    println!("Starting Timelapse Creator Backend on http://0.0.0.0:8080");
    
    HttpServer::new(move || {
        let job_store = web::Data::new(job_store.clone());
        
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        App::new()
            .wrap(cors)
            .app_data(job_store.clone())
            .service(
                web::scope("/api")
                    .route("/upload", web::post().to(upload_files))
                    .route("/preview/{job_id}/{index}", web::get().to(get_preview))
                    .route("/create-timelapse", web::post().to(create_timelapse_handler))
                    .route("/job-status/{job_id}", web::get().to(get_job_status))
                    .route("/download/{job_id}", web::get().to(download_video))
            )
            .route("/health", web::get().to(health_check))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
