use cors::Cors;
use models::projects::Project;
use params::create_project_params::CreateProjectParams;
use rocket::{serde::{self, json::Json}, response::{status::Custom, content::RawJson}, http::Status, Data, data::ToByteUnit, tokio::io, fs::NamedFile};
use session_db::SessionDatabase;

#[macro_use] extern crate rocket;

mod models;
mod session_db;
mod params;
mod cors;

#[get("/user/<username>")]
async fn get_user_projects(username: String) -> Custom<RawJson<String>>{
    match SessionDatabase::get_user(&username).await {
        Some(usr) => match serde::json::to_string(&usr.get_projects().await) {
            Ok(val) => Custom(Status::Ok, RawJson(val)),
            Err(err) => return Custom(Status::InternalServerError, RawJson(format!("{{ \"error\": \"{}\" }}", err)))
        }
        None => return Custom(Status::NotFound, RawJson("{ \"error\": \"User not found\" }".to_string())),
    }
}

#[get("/<project_id>")]
async fn get_project(project_id: String) -> Custom<RawJson<String>> {
    match SessionDatabase::get_project(&project_id).await {
        Some(proj) => match serde::json::to_string(&proj) {
            Ok(val) => return Custom(Status::Ok, RawJson(val)),
            Err(err) => return Custom(Status::InternalServerError, RawJson(format!("{{ \"error\": \"{}\" }}", err)))
        },
        None => return Custom(Status::NotFound, RawJson("{ \"error\": \"Project not found\"}".to_string())),
    }
}

#[post("/create_project", data="<params>")]
async fn create_project(params: Json<CreateProjectParams>) -> Custom<RawJson<String>> {
    let project = SessionDatabase::create_project(&params.username, &params.title, &params.description).await;
    Custom(Status::Ok, RawJson(serde::json::to_string(&project).unwrap()))
}

#[post("/save_project", data="<params>")]
async fn save_project(params: Json<Project>) -> Status {
    let res = SessionDatabase::save_project(&params.0).await;

    if res.is_ok() {
        return Status::Ok;
    }

    Status::InternalServerError
}

#[get("/delete/<project_id>")]
async fn delete_project(project_id: String) -> Status {
    let res = SessionDatabase::delete_project(&project_id).await;

    if res.is_ok() {
        return Status::Ok;
    }

    Status::InternalServerError
}

#[post("/upload/<project_id>/<filename>", data="<audio>")]
async fn upload_audio(project_id: String, filename: String, audio: Data<'_>) -> io::Result<()> {
    let mut bytes = Vec::new();
    audio.open(24.megabytes())
        .stream_to(&mut bytes)
        .await?;

    SessionDatabase::save_audio(&project_id, &filename, &bytes).await?;

    Ok(())
}

#[get("/<project_id>/<filename>")]
async fn download_audio(project_id: String, filename: String) -> Option<NamedFile> {
    SessionDatabase::get_audio(&project_id, &filename).await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .mount("/project/", routes![get_user_projects, get_project, create_project, save_project, delete_project])
        .mount("/audio/", routes![upload_audio, download_audio])
}

#[cfg(test)]
mod test {
}
