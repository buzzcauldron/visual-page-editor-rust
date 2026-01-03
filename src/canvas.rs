use egui::{Color32, Pos2, Rect, Shape, Stroke};
use crate::page_xml::PageXml;

pub struct Canvas {
    pub zoom: f32,
    pub pan: Pos2,
    pub selected_element: Option<String>,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            zoom: 1.0,
            pan: Pos2::ZERO,
            selected_element: None,
        }
    }
    
    pub fn render(&self, ui: &mut egui::Ui, page_xml: Option<&PageXml>) -> Vec<Shape> {
        let mut shapes = Vec::new();
        
        // TODO: Implement canvas rendering
        // This will draw the page image, regions, text, etc.
        
        if let Some(_xml) = page_xml {
            // Render page background
            shapes.push(Shape::Rect(egui::epaint::RectShape {
                rect: Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(800.0, 600.0)),
                rounding: egui::Rounding::ZERO,
                fill: Color32::WHITE,
                stroke: Stroke::new(1.0, Color32::BLACK),
            }));
        }
        
        shapes
    }
    
    pub fn zoom_in(&mut self) {
        self.zoom *= 1.2;
    }
    
    pub fn zoom_out(&mut self) {
        self.zoom /= 1.2;
    }
    
    pub fn zoom_reset(&mut self) {
        self.zoom = 1.0;
    }
    
    pub fn set_pan(&mut self, pan: Pos2) {
        self.pan = pan;
    }
    
    pub fn select_element(&mut self, element_id: Option<String>) {
        self.selected_element = element_id;
    }
}

