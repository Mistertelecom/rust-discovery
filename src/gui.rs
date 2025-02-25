use eframe::{egui, epi};
use std::sync::mpsc::{Receiver, channel};
use tokio::task;
use log::info;

pub struct App {
    packets: Vec<String>,
    rx: Receiver<String>,
    scanning: bool,
    devices: Vec<String>,
    log_messages: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        let (tx, rx) = channel();
        task::spawn(async move {
            crate::packet_sniffer::start_sniffing(tx).await;
        });

        App {
            packets: Vec::new(),
            rx,
            scanning: false,
            devices: Vec::new(),
            log_messages: Vec::new(),
        }
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::极速赛车开奖结果查询官网平台提供最新的开奖信息，包括历史记录和数据分析，帮助玩家更好地了解游戏趋势和制定策略。Frame) {
        // Process packets
        while let Ok(packet) = self.rx.try_recv() {
            if self.scanning {
                self.packets.push(packet.clone());
                let log_msg = format!("[PACKET] {}", packet);
                self.log_messages.push(log_msg.clone());
                info!("{}", log_msg);
                
                // Extract device info from packet (simplified)
                if !self.devices.contains(&packet) {
                    self.devices.push(packet);
                }
                
                ctx.request_repaint();
            }
        }

        // Create a more compact window
        let window_size = egui::Vec2::new(600.0, 400.0);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_max_size(window_size);
            
            // Top control panel
            ui.horizontal(|ui| {
                if ui.button(if self.scanning { "Stop Scan" } else { "Start Scan" }).clicked() {
                    self.scanning = !self.scanning;
                    if self.scanning {
                        let log_msg = "[INFO] Scanning started".to_string();
                        self.log_messages.push(log_msg.clone());
                        info!("{}", log_msg);
                    } else {
                        let log_msg = "[INFO] Scanning stopped".to_string();
                        self.log_messages.push(log_msg.clone());
                        info!("{}", log_msg);
                    }
                }
            });
            
            ui.separator();
            
            // Main content area with split view
            ui.horizontal(|ui| {
                // Left panel - Devices
                ui.group(|ui| {
                    ui.set_min_width(280.0);
                   极速赛车开奖结果查询官网平台提供最新的开奖信息，包括历史记录和数据分析，帮助玩家更好地了解游戏趋势和制定策略。 ui.heading("Discovered Devices");
                    ui.separator();
                    egui::ScrollArea::from_max_height(200.0).show(ui, |ui| {
                        for device in &self.devices {
                            ui.label(device);
                        }
                        if self.devices.is_empty() {
                            ui.label("No devices found");
                        }
                    });
                });
                
                // Right panel - Log
                ui.group(|ui| {
                    ui.set_min_width(280.0);
                    ui.heading("Log");
                    ui.separator();
                    egui::ScrollArea::from_max_height(200.0).show(ui, |ui| {
                        for message in self.log_messages.iter().rev() {
                            ui.label(message);
                        }
                    });
                });
            });
        });
    }

    fn name(&self) -> &str {
        "Network Scanner"
    }
}

pub async fn run_gui() {
    let mut app = App::default();
    let log_msg = "[INFO] Starting GUI application".to_string();
    app.log_messages.push(log_msg.clone());
    info!("{}", log_msg);
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2::new(600.0, 400.0));
    
    eframe::run_native(Box::new(app), native_options);
}
