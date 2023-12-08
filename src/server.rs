use crate::project::{get_project_manager, ProjectManager};
use crate::connections::Connection;
use crate::commands::{GodataCommand, ManagementCommand, ProjectCommand};
use crate::routes;

use std::sync::{Arc, Mutex};
use std::io::Result;
use tokio_stream::wrappers::UnixListenerStream;



pub struct Server {
    state_manger: ServerState
}

struct ServerState {
    project_manager: Arc<Mutex<ProjectManager>>
}


impl Server {
    pub async fn start(&self) {
        let listener = tokio::net::UnixListener::bind("/tmp/godata.sock").unwrap();
        let incoming = UnixListenerStream::new(listener);
        warp::serve(routes::routes(self.state_manger.project_manager.clone()))
            .run_incoming(incoming)
            .await;
    }
}

pub fn get_server() -> Server {
    let server = Server {
        state_manger: ServerState {
            project_manager: Arc::new(Mutex::new(get_project_manager()))
        }
    };
    server
}
