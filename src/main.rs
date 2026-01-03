use eframe::egui;
use log::info;

mod app;
mod page_xml;
mod canvas;
mod ui;

use app::VisualPageEditorApp;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    
    info!("Starting Visual Page Editor (Rust)");
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Visual Page Editor"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Visual Page Editor",
        options,
        Box::new(|cc| {
            // Setup custom fonts if needed
            setup_custom_styles(&cc.egui_ctx);
            
            Box::new(VisualPageEditorApp::new(cc))
        }),
    )
}

fn setup_custom_styles(ctx: &egui::Context) {
    use egui::{FontFamily, FontId, TextStyle};
    
    let mut style = (*ctx.style()).clone();
    
    // Configure text styles
    style.text_styles.insert(
        TextStyle::Heading,
        FontId::new(24.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Body,
        FontId::new(14.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Monospace,
        FontId::new(12.0, FontFamily::Monospace),
    );
    
    ctx.set_style(style);
}

