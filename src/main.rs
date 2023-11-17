use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Debug, Serialize, Deserialize)]
struct AccessData {
    // Define the struct fields here
}

#[derive(Debug, Serialize, Deserialize)]
struct IpfsMetaData {
    // Define the struct fields here
}

#[actix_web::get("/api/file/node/status")]
async fn get_status() -> impl Responder {
    // Implement the logic for the getStatus function
    // ...

    HttpResponse::Ok().json(json!({ "isClusterOnline": true }))
}

#[actix_web::get("/api/file/view/access-play/{accessKey}/{token}")]
async fn play_video(info: web::Path<(String, String)>) -> impl Responder {
    // Implement the logic for the playVideo function
    // ...

    HttpResponse::Ok().json(json!({ "message": "Video playing" }))
}

#[actix_web::get("/api/file/view/access/{accessKey}/{token}")]
async fn get_access_file(info: web::Path<(String, String)>) -> impl Responder {
    // Implement the logic for the getAcessFile function
    // ...

    HttpResponse::Ok().body("File content")
}

#[actix_web::get("/api/file/download/{accessKey}/{token}")]
async fn download_file(info: web::Path<(String, String)>) -> impl Responder {
    // Implement the logic for the downloadFile function
    // ...

    HttpResponse::Ok().body("File content for download")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_status)
            .service(play_video)
            .service(get_access_file)
            .service(download_file)
    })
    .bind("127.0.0.1:3008")?
    .run()
    .await
}
