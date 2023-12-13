
use crate::project::ProjectManager;
use crate::project::get_collection_names;
use std::sync::{Arc, Mutex};
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::reply::WithStatus;
use std::collections::HashMap;

pub(crate) fn list_collections(show_hidden: bool) -> Result<impl warp::Reply, Infallible> {
    let collections = get_collection_names(show_hidden);
    Ok(warp::reply::json(&collections.unwrap()))
}

pub(crate) fn list_projects(project_manager: Arc<Mutex<ProjectManager>>, collection: String, show_hidden: bool) -> Result<impl warp::Reply, Infallible> {
    let projects = project_manager.lock().unwrap().get_project_names(collection.clone(), show_hidden);
    match projects {
        Ok(project_list) => Ok(warp::reply::with_status(warp::reply::json(&project_list), StatusCode::OK)),
        Err(_) => Ok(warp::reply::with_status(warp::reply::json(
            &format!("No collection named {collection}")
        ), StatusCode::NOT_FOUND))
    }
}



pub(crate) fn list_project(project_manager: Arc<Mutex<ProjectManager>>, collection: String,  project_name: String, project_path: Option<String>, _show_hidden: bool) -> Result<impl warp::Reply, Infallible> {
    let project = project_manager.lock().unwrap().load_project(&project_name, &collection);
    if project.is_ok() {
        let project = project.unwrap();
        let result = project.lock().unwrap().list(project_path);
        match result {
            Ok(list) => return Ok(warp::reply::with_status(warp::reply::json(&list), StatusCode::OK)),
            Err(_) => return Ok(warp::reply::with_status(warp::reply::json(
                &"Path does not exist!".to_string()
            ), StatusCode::NOT_FOUND))
        }
    }
    Ok(warp::reply::with_status(warp::reply::json(
        &format!("No project named {project_name} in collection {collection}")
    ), StatusCode::NOT_FOUND))
}

pub(crate) fn create_project(project_manager: Arc<Mutex<ProjectManager>>, collection: String, project_name: String, force:bool, storage_location:Option<String>) -> Result<impl warp::Reply, Infallible> {
    let project = project_manager.lock().unwrap().create_project(&project_name, &collection, force, storage_location);
    match project {
        Ok(_) => Ok(warp::reply::with_status(warp::reply::json(
            &format!("Project {project_name} created in collection {collection}")
        ), StatusCode::CREATED)),
        Err(e) => Ok(warp::reply::with_status(warp::reply::json(
            &e.to_string()
        ), StatusCode::CONFLICT))
    }
}

pub(crate) fn delete_project(project_manager: Arc<Mutex<ProjectManager>>, collection: String, project_name: String, force: bool) -> Result<impl warp::Reply, Infallible> {
    let project = project_manager.lock().unwrap().delete_project(&project_name, &collection, force);
    match project {
        Ok(_) => Ok(warp::reply::with_status(warp::reply::json(
            &format!("Project {project_name} deleted from collection {collection}")
        ), StatusCode::OK)),
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::InvalidData => {
                    Ok(warp::reply::with_status(warp::reply::json(
                      &e.to_string()),
                     StatusCode::FORBIDDEN))
                },
                _ => Ok(warp::reply::with_status(warp::reply::json(
                    &e.to_string()),
                    StatusCode::NOT_FOUND))
            }
        }    
    }
}

pub(crate) fn link_file(project_manager: Arc<Mutex<ProjectManager>>, collection: String, project_name: String, project_path: String, file_path: String, force: bool) -> Result<WithStatus<warp::reply::Json>,  Infallible> {
    let project = project_manager.lock().unwrap().load_project(&project_name, &collection);
    if project.is_ok() {
        let project = project.unwrap();
        let result = project.lock().unwrap().add_file(&project_path, &file_path, force);
        match result {
            Ok(r) => {
                let pervious_path = r.0;
                let was_internal = r.1;
                let mut output: HashMap<String, _> = HashMap::new();
                output.insert("overwritten".to_string(), 
                                pervious_path.map_or("none".to_string(), |path| {
                                    if was_internal{
                                        path
                                    }
                                    else{
                                        "none".to_string()
                                    }
                                }));
                output.insert("message".to_string(), format!("File {file_path} linked to {project_path} in project {project_name} in collection {collection}"));

                return Ok(warp::reply::with_status(warp::reply::json(&output), StatusCode::CREATED));
            },   
            Err(e) => return Ok(warp::reply::with_status(
                warp::reply::json(&e.to_string()),
                StatusCode::CONFLICT)),
        }
    }
    Ok(warp::reply::with_status(
       warp::reply::json(&format!("No project named {project_name} in collection {collection}")),
       StatusCode::NOT_FOUND))

}

pub(crate) fn link_folder(project_manager: Arc<Mutex<ProjectManager>>, collection: String, project_name: String, project_path: String, folder_path: String, recursive: bool) -> Result<WithStatus<warp::reply::Json
>,  Infallible> {
    let project = project_manager.lock().unwrap().load_project(&project_name, &collection);
    if project.is_ok() {
        let project = project.unwrap();
        let result = project.lock().unwrap().add_folder(&project_path, &folder_path, recursive);
        match result {
            Ok(_) => {
                let mut out = HashMap::new();
                out.insert("message".to_string(), format!("Folder {folder_path} linked to {project_path} in project {project_name} in collection {collection}"));
                out.insert("overwritten".to_string(), "none".to_string());
                return Ok(warp::reply::with_status(
                warp::reply::json(&out),
                StatusCode::CREATED))
            },
 
            Err(e) => return Ok(warp::reply::with_status(
                warp::reply::json(&e.to_string()),
                StatusCode::NOT_FOUND)),
        }
    }
    Ok(warp::reply::with_status(
       warp::reply::json(&format!("No project named {project_name} in collection {collection}")),
       StatusCode::NOT_FOUND))

}

pub(crate) fn get_file(project_manager: Arc<Mutex<ProjectManager>>, collection: String, project_name: String, project_path: String) -> Result<WithStatus<warp::reply::Json>,  Infallible> {
    let project = project_manager.lock().unwrap().load_project(&project_name, &collection);
    if project.is_ok() {
        let project = project.unwrap();
        let result = project.lock().unwrap().get_file(&project_path);
        match result {
            Ok(file) => return Ok(warp::reply::with_status(
                warp::reply::json(&file),
                StatusCode::OK)),
 
            Err(_) => return Ok(warp::reply::with_status(
                warp::reply::json(&format!("File {project_path} does not exist!")),
                StatusCode::NOT_FOUND)),
        }
    }
    Ok(warp::reply::with_status(
       warp::reply::json(&format!("No project named {project_name} in collection {collection}")),
       StatusCode::NOT_FOUND))

}

pub(crate) fn generate_path(project_manager: Arc<Mutex<ProjectManager>>, collection: String, project_name: String, project_path: String) -> Result<WithStatus<warp::reply::Json>,  Infallible> {
    let project = project_manager.lock().unwrap().load_project(&project_name, &collection);
    if project.is_ok() {
        let project = project.unwrap();
        let result = project.lock().unwrap().generate_path(&project_path);
        match result {
            Ok(path) => return Ok(warp::reply::with_status(
                warp::reply::json(&path),
                StatusCode::OK)),
 
            Err(_) => return Ok(warp::reply::with_status(
                warp::reply::json(&"Uncaught error generating path!".to_string()),
                StatusCode::INTERNAL_SERVER_ERROR)),
        }
    };
    
    Ok(warp::reply::with_status(
       warp::reply::json(&format!("No project named {project_name} in collection {collection}")),
       StatusCode::NOT_FOUND))
    }

pub(crate) fn path_exists(project_manager: Arc<Mutex<ProjectManager>>, collection: String, project_name: String, project_path: String) -> Result<WithStatus<warp::reply::Json>,  Infallible> {
    let project = project_manager.lock().unwrap().load_project(&project_name, &collection);
    if project.is_ok() {
        let project = project.unwrap();
        let result = project.lock().unwrap().exists(project_path);
        if result{
            return Ok(warp::reply::with_status(
                warp::reply::json(&true),
                StatusCode::OK))
        }
        else {
            return Ok(warp::reply::with_status(
                warp::reply::json(&false),
                StatusCode::OK))
        }
    }
    Ok(warp::reply::with_status(
       warp::reply::json(&format!("No project named {project_name} in collection {collection}")),
       StatusCode::NOT_FOUND))

}

pub(crate) fn remove_file(project_manager: Arc<Mutex<ProjectManager>>, collection: String, project_name: String, project_path: String) -> Result<WithStatus<warp::reply::Json>,  Infallible> {
    let project = project_manager.lock().unwrap().load_project(&project_name, &collection);
    if project.is_ok() {
        let project = project.unwrap();
        let result = project.lock().unwrap().remove_file(&project_path);
        match result {
            Ok(_) => return Ok(warp::reply::with_status(
                warp::reply::json(&format!("File {project_path} removed from project {project_name} in collection {collection}")),
                StatusCode::OK)),
 
            Err(_) => return Ok(warp::reply::with_status(
                warp::reply::json(&format!("File {project_path} does not exist!")),
                StatusCode::NOT_FOUND)),
        }
    }
    Ok(warp::reply::with_status(
       warp::reply::json(&format!("No project named {project_name} in collection {collection}")),
       StatusCode::NOT_FOUND))

}