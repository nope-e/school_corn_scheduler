use std::time::Duration;
use eframe::epaint::Color32;
use eframe::Frame;
use egui::{Context, RichText};
use tokio::sync::{mpsc, oneshot};

pub(crate) struct ModalRequest{
    pub(crate) task:String,
    pub(crate) eta:Duration,
    pub(crate) resp_tx:oneshot::Sender<bool>,
}

pub(crate) struct MyApp {
    task: String,
    eta:Duration,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            task: "默认任务".to_owned(),
            eta: Duration::from_secs(30),
        }
    }
}

// Demonstrates how to replace all fonts.
fn replace_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "MiSans-Regular".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../fonts/MiSans-Regular.ttf"
        ))),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "MiSans-Regular".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("MiSans-Regular".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

impl MyApp {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        replace_fonts(&cc.egui_ctx);
        Self {
            ..Self::default()
        }
    }
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(RichText::new("定时任务预告").color(Color32::RED));
            ui.horizontal(|ui| {
                // 移除组件之间的默认间距（可选，如果你希望文字紧凑连贯）
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label(RichText::new(format!("{} 秒", self.eta.as_secs_f32())).color(Color32::RED));
                ui.label(RichText::new("后，将执行 ").color(Color32::WHITE));
                ui.label(RichText::new(self.task.clone()).color(Color32::GREEN));
            });
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button(RichText::new("确定").color(Color32::GREEN)).clicked() {
                    log::info!("点击了确定")

                }
                if ui.button(RichText::new("取消").color(Color32::RED)).clicked() {
                    log::info!("点击了取消")
                }
            });

            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));
        });
    }
}
