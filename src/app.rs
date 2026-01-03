use eframe::egui;
use std::path::PathBuf;
use crate::page_xml::PageXml;
use crate::canvas::Canvas;
use crate::ui;

pub struct VisualPageEditorApp {
    pub current_file: Option<PathBuf>,
    pub page_xml: Option<PageXml>,
    pub canvas: Canvas,
    pub file_list: Vec<PathBuf>,
    pub current_index: usize,
    pub show_menu: bool,
}

impl VisualPageEditorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            current_file: None,
            page_xml: None,
            canvas: Canvas::new(),
            file_list: Vec::new(),
            current_index: 0,
            show_menu: true,
        }
    }
}

impl eframe::App for VisualPageEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard shortcuts
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Escape) {
                self.show_menu = !self.show_menu;
            }
            if i.key_pressed(egui::Key::O) && i.modifiers.ctrl {
                self.open_file_dialog();
            }
            if i.key_pressed(egui::Key::S) && i.modifiers.ctrl {
                self.save_file();
            }
            if i.key_pressed(egui::Key::PageUp) {
                self.load_previous();
            }
            if i.key_pressed(egui::Key::PageDown) {
                self.load_next();
            }
        });
        
        // Main UI
        egui::CentralPanel::default().show(ctx, |ui| {
            ui::render_main_ui(ui, self);
        });
        
        // Menu bar
        if self.show_menu {
            egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                ui::render_menu_bar(ui, self);
            });
        }
        
        // Status bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui::render_status_bar(ui, self);
        });
    }
}

impl VisualPageEditorApp {
    pub fn open_file_dialog(&mut self) {
        // TODO: Implement file dialog
        // For now, this is a placeholder
    }
    
    pub fn save_file(&mut self) {
        if let Some(ref file) = self.current_file {
            if self.page_xml.is_some() {
                // TODO: Implement save functionality
                log::info!("Saving file: {:?}", file);
            }
        }
    }
    
    fn load_previous(&mut self) {
        if !self.file_list.is_empty() && self.current_index > 0 {
            self.current_index -= 1;
            self.load_file(self.file_list[self.current_index].clone());
        }
    }
    
    fn load_next(&mut self) {
        if !self.file_list.is_empty() && self.current_index < self.file_list.len() - 1 {
            self.current_index += 1;
            self.load_file(self.file_list[self.current_index].clone());
        }
    }
    
    fn load_file(&mut self, path: PathBuf) {
        match PageXml::from_file(&path) {
            Ok(xml) => {
                self.current_file = Some(path);
                self.page_xml = Some(xml);
                log::info!("Loaded file: {:?}", self.current_file);
            }
            Err(e) => {
                log::error!("Failed to load file: {}", e);
            }
        }
    }
}

