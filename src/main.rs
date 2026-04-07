#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

mod modal;

use eframe::{egui};
#[tokio::main]
async fn main() -> eframe::Result {

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([250.0, 100.0]).with_position([1780.0, 1000.0]).with_decorations(false).with_always_on_top().with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "提示",
        options,
        Box::new(|cc| Ok(Box::new(modal::MyApp::new(cc)))),
    )
}


