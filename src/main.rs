pub mod packet_sniffer;
mod gui;

use tokio::runtime::Runtime;
use gui::run_gui;
use env_logger::Env;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        run_gui().await;
    });
}
