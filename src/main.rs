use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use commands::find_by_login;
use std::fs::File;
use std::path::PathBuf;

//mod commands;
mod commands;
mod input_output;
mod utils;

//use crate::commands::edit_password;
use crate::utils::set_path;

const SECRET_FILE_NAME: &str = "secret_passwd";
pub const GLOBAL_SECRET_KEY: &str = "29/Renenal";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //    let arg: String = "Box".to_string();
    //    find_by_login(&arg, file_path);

    // Créer un serveur HTTP Actix
    HttpServer::new(|| App::new().route("/search/{arg}", web::get().to(search_handler)))
        .bind("127.0.0.1:8080")? // Remplacez par votre adresse IP et port souhaités
        .run()
        .await
}

fn search(arg: &str) -> String {
    let secret_file_name = SECRET_FILE_NAME;
    let file_path: PathBuf = set_path(secret_file_name.to_string());
    if !file_path.exists() {
        if let Err(e) = File::create(&file_path) {
            println!("could not create file: {e}");
        };
    }
    let response = find_by_login(&arg, file_path);
    response
}

// Handler pour gérer la requête HTTP avec la fonction search
async fn search_handler(info: web::Path<String>) -> impl Responder {
    let result = search(&info);
    HttpResponse::Ok().body(result)
}
