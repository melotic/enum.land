use crate::backends::crtsh::CrtshClient;

pub struct AppState {
    pub crtsh_client: Box<dyn CrtshClient + Sync + Send>,
}
