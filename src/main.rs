mod packet_sniffer;
mod gui;

use tokio::runtime::Runtime;
use gui::run_gui;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        run_gui().await;
    });
}