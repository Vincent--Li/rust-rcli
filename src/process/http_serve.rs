use std::path::Path;

use tracing::info;

pub fn process_http_serve(path: &Path, port: u16) {
    info!("Serving {:?} on port {}", path, port);
}
