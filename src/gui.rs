use eframe::{egui, epi};
use std::sync::mpsc::{Receiver, channel};
use tokio::task;
use crate::packet_sniffer::start_sniffing;

pub struct App {
    packets: Vec<String>,
    rx: Receiver<String>,
}

impl Default for App {
    fn default() -> Self {
        let (tx, rx) = channel();
        task::spawn(async move {
            start_sniffing(tx).await;
        });

        App {
            packets: Vec::new(),
            rx,
        }
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame) {
        while let Ok(packet) = self.rx.try_recv() {
            self.packets.push(packet);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::auto_sized().show(ui, |ui| {
                for packet in &self.packets {
                    ui.label(packet);
                }
            });
        });
    }

    fn name(&self) -> &str {
        "Network Sniffer"
    }
}

pub async fn run_gui() {
    let app = App::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}