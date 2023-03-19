use std::path::Path;
use rand::Rng;

use rocket::{tokio::{fs::{self, File}, io::{AsyncWriteExt, self}}, serde, fs::NamedFile};
use crate::models::{user::User, projects::Project};

// TODO 
// THIS ENTIRE THING IS NOT SCALABLE
// TEMPORARY SOLUTION
// NEED TO FIX
pub struct SessionDatabase;
impl SessionDatabase {
    const USERS_PATH: &str = "data/users.json";
    const PROJECTS_PATH: &str = "data/projects/";

    pub async fn get_project(project_id: &str) -> Option<Project> {
        let project_path = SessionDatabase::PROJECTS_PATH.to_string() + project_id;
        let project_path = Path::new(&project_path);

        if !project_path.exists() {
            return None;
        }

        let root_content = fs::read_to_string(project_path.join("root.json"))
            .await
            .unwrap();

        Some(serde::json::from_str::<Project>(&root_content).unwrap()) 
    }

    pub async fn create_project(username: &str, title: &str, description: &str) -> Project { 
        let project = Project {
            project_id: SessionDatabase::random_id(),
            title: title.to_string(),
            description: description.to_string(), 
            sound_cues: vec![], 
            scenes: vec![],
        };

        let project_path = SessionDatabase::PROJECTS_PATH.to_string() + &project.project_id;
        let project_path = Path::new(&project_path);

        fs::create_dir(&project_path).await.unwrap();

        let mut root_project = File::create(project_path.join("root.json")).await.unwrap();
        root_project.write(serde::json::to_pretty_string(&project).unwrap().as_bytes()).await.unwrap();

        let mut users = SessionDatabase::get_users().await;
        if let Some(current_user) = users.iter_mut().find(|e| e.username.to_string() == username.to_string()) {
            current_user.projects.push(project.project_id.clone());
        }
        fs::write(SessionDatabase::USERS_PATH, serde::json::to_pretty_string(&users).unwrap()).await.unwrap();

        project
    }

    pub async fn save_project(project: &Project) -> io::Result<()> {
        let project_path = SessionDatabase::PROJECTS_PATH.to_string() + &project.project_id;
        let project_path = Path::new(&project_path);

        fs::write(project_path.join("root.json"), serde::json::to_pretty_string(project).unwrap())
            .await?;

        Ok(())
    }

    pub async fn delete_project(project_id: &str) -> io::Result<()>{
        let project_path = SessionDatabase::PROJECTS_PATH.to_string() + project_id;
        let project_path = Path::new(&project_path);
        
        fs::remove_dir_all(project_path).await?;

        // Find the user with this project and delete this project from user then save updated user
        // data
        let mut users = SessionDatabase::get_users().await;
        if let Some(current_user) = users.iter_mut().find(|e| e.projects.contains(&project_id.to_string())) {
            current_user.projects.retain(|e| e != project_id);
        }
        fs::write(SessionDatabase::USERS_PATH, serde::json::to_pretty_string(&users).unwrap()).await.unwrap();

        Ok(())
    }

    pub async fn save_audio(project_id: &str, filename: &str, audio: &Vec<u8>) -> io::Result<()> {
        let project_path = SessionDatabase::PROJECTS_PATH.to_string() + project_id;
        let project_path = Path::new(&project_path).join(filename);

        let mut file = File::create(project_path).await?;
        file.write_all(audio).await?;

        Ok(())
    }

    pub async fn get_audio(project_id: &str, filename: &str) -> Option<NamedFile> {
        let project_path = SessionDatabase::PROJECTS_PATH.to_string() + project_id;
        let project_path = Path::new(&project_path).join(filename);
        NamedFile::open(project_path).await.ok()
    }

    pub async fn get_user(username: &str) -> Option<User> {
        SessionDatabase::get_users()
            .await
            .iter()
            .find(|e| e.username == username)
            .cloned()
    }

    pub async fn get_users() -> Vec<User> {
        serde::json::from_str::<Vec<User>>(&fs::read_to_string(SessionDatabase::USERS_PATH).await.unwrap()).unwrap()
    }

    fn random_id() -> String {
        let mut result = String::new();
        
        for _ in 0..16 {
            result.push(rand::thread_rng().gen_range(97..123) as u8 as char);
        }

        result
    }
}
