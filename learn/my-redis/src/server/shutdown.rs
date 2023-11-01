use tokio::sync::broadcast;

#[derive(Debug)]
pub struct Shutdown {
    pub is_shutdown: bool,
    pub notify: broadcast::Receiver<()>,
}
