use cors::Cors;
use models::projects::Project;
use params::create_project_params::CreateProjectParams;
use rocket::{routes, get, post, launch, serde::{json::Json}, response::{status::Custom, content::RawJson}, http::Status, Data, data::ToByteUnit, tokio::io, fs::NamedFile, figment::{Figment, providers::{Toml, Format}}, Config, State};
use crate::session_db::{DbFairing, EchoDatabase};

mod models;
mod session_db;
mod params;
mod cors;

#[get("/user/<username>")]
async fn get_user_projects(username: String, db: &State<EchoDatabase>) -> Custom<RawJson<String>>{
    match db.query("SELECT * from ", None).await {
        Ok(_) => {}
        Err(_) => {}
    };
    
    return Custom(Status::NotFound, RawJson("{ \"error\": \"User not found\" }".to_string()));
}

#[get("/<project_id>")]
async fn get_project(project_id: String, db: &State<EchoDatabase>) -> Custom<RawJson<String>> {
    match db.query("").await {
        Ok(_) => {}
        Err(_) => {}
    };
    
    return Custom(Status::NotFound, RawJson("{ \"error\": \"User not found\" }".to_string()));
}

#[post("/create_project", data="<params>")]
async fn create_project(params: Json<CreateProjectParams>, db: &State<EchoDatabase>) -> Custom<RawJson<String>> {
    // let project = SessionDatabase::create_project(&params.username, &params.title, &params.description).await;
    // Custom(Status::Ok, RawJson(serde::json::to_string(&project).unwrap()))
    
    return Custom(Status::NotFound, RawJson("{ \"error\": \"User not found\" }".to_string()));
}

#[post("/save_project", data="<params>")]
async fn save_project(params: Json<Project>, db: &State<EchoDatabase>) -> Status {
    // TODO save
    
    // if res.is_ok() {
    //     return Status::Ok;
    // }

    Status::InternalServerError
}

#[get("/delete/<project_id>")]
async fn delete_project(project_id: String, db: &State<EchoDatabase>) -> Status {
    // TODO delete
    
    // if res.is_ok() {
    //     return Status::Ok;
    // }

    Status::InternalServerError
}

#[post("/upload/<project_id>/<filename>", data="<audio>")]
async fn upload_audio(project_id: String, filename: String, audio: Data<'_>, db: &State<EchoDatabase>) -> io::Result<()> {
    let mut bytes = Vec::new();
    audio.open(24.megabytes())
        .stream_to(&mut bytes)
        .await?;

    // TODO save audio
    
    Ok(())
}

#[get("/<project_id>/<filename>")]
async fn download_audio(project_id: String, filename: String, db: &State<EchoDatabase>) -> Option<NamedFile> {
    // TODO send audio
    None
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/project/", routes![get_user_projects, get_project, create_project, save_project, delete_project])
        .mount("/audio/", routes![upload_audio, download_audio])
        .attach(Cors)
        .attach(DbFairing)
}

#[cfg(test)]
mod test {
}