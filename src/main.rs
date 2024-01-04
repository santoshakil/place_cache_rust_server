pub mod db;
pub mod logs;
pub mod place;
pub mod server_tonic;
pub mod server_ws;

#[tokio::main]
async fn main() {
    if let Err(err) = logs::init_log4rs() {
        eprintln!("Error initializing log4rs: {}", err);
    }

    if let Err(err) = db::init_db().await {
        log::error!("Error initializing database: {}", err);
    }

    let ws_thread = tokio::task::spawn(server_ws::start_server_ws());
    let tonic_thread = tokio::task::spawn(server_tonic::start_server_tonic());

    let join = tokio::join!(ws_thread, tonic_thread);

    if let Err(err) = join.0 {
        log::error!("Error running websocket server: {}", err);
    }

    if let Err(err) = join.1 {
        log::error!("Error running tonic server: {}", err);
    }
}
