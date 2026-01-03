use eframe::egui;
use crate::app::VisualPageEditorApp;

pub fn render_menu_bar(ui: &mut egui::Ui, app: &mut VisualPageEditorApp) {
    egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("Open...").clicked() {
                app.open_file_dialog();
                ui.close_menu();
            }
            if ui.button("Save").clicked() {
                app.save_file();
                ui.close_menu();
            }
            if ui.button("Save As...").clicked() {
                // TODO: Implement save as
                ui.close_menu();
            }
            ui.separator();
            if ui.button("Exit").clicked() {
                std::process::exit(0);
            }
        });
        
        ui.menu_button("Edit", |ui| {
            if ui.button("Undo").clicked() {
                // TODO: Implement undo
                ui.close_menu();
            }
            if ui.button("Redo").clicked() {
                // TODO: Implement redo
                ui.close_menu();
            }
        });
        
        ui.menu_button("View", |ui| {
            if ui.button("Zoom In").clicked() {
                app.canvas.zoom_in();
                ui.close_menu();
            }
            if ui.button("Zoom Out").clicked() {
                app.canvas.zoom_out();
                ui.close_menu();
            }
            if ui.button("Reset Zoom").clicked() {
                app.canvas.zoom_reset();
                ui.close_menu();
            }
        });
        
        ui.menu_button("Help", |ui| {
            if ui.button("About").clicked() {
                // TODO: Show about dialog
                ui.close_menu();
            }
        });
    });
}

pub fn render_main_ui(ui: &mut egui::Ui, app: &mut VisualPageEditorApp) {
    ui.vertical_centered(|ui| {
        ui.heading("Visual Page Editor (Rust)");
        
        if app.current_file.is_none() {
            ui.label("No file loaded. Use File > Open to load a Page XML file.");
        } else {
            ui.label(format!("File: {:?}", app.current_file.as_ref().unwrap()));
            
            // Canvas area
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let shapes = app.canvas.render(ui, app.page_xml.as_ref());
                ui.painter().extend(shapes);
            });
        }
    });
}

pub fn render_status_bar(ui: &mut egui::Ui, app: &VisualPageEditorApp) {
    ui.horizontal(|ui| {
        if let Some(ref file) = app.current_file {
            ui.label(format!("File: {}", file.display()));
        } else {
            ui.label("No file");
        }
        
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if !app.file_list.is_empty() {
                ui.label(format!(
                    "Page {} of {}",
                    app.current_index + 1,
                    app.file_list.len()
                ));
            }
        });
    });
}

