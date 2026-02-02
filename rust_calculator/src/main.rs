#![windows_subsystem = "windows"]
use eframe::egui;
use eframe::{App, Frame};
use meval::eval_str;

struct Calculator {
    input: String,
    result: f64,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            input: String::new(),
            result: 0.0,
        }
    }
}

impl App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(5.0);
                ui.label(egui::RichText::new("MATH+").strong().color(egui::Color32::GRAY));
                
                // --- FIXED DISPLAY AREA ---
                // allocate_ui ensures this block doesn't grow and hide the keypad
                ui.allocate_ui(egui::vec2(260.0, 100.0), |ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        ui.set_height(100.0); 
                        ui.set_width(260.0);
                        
                        ui.vertical(|ui| {
                            ui.add_space(10.0);
                            
                            // Current Input
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.add_space(10.0);
                                ui.label(egui::RichText::new(&self.input).size(32.0).monospace());
                            });

                            // Calculated Result
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.add_space(10.0);
                                let res_text = if self.result.is_nan() { 
                                    "Error".to_string() 
                                } else { 
                                    format!("= {}", self.result) 
                                };
                                ui.label(egui::RichText::new(res_text).size(18.0).color(egui::Color32::LIGHT_BLUE));
                            });
                        });
                    });
                });

                ui.add_space(10.0);
                ui.separator(); 
                ui.add_space(10.0);

                // --- BUTTON GRID (centered, fixed width) ---
                ui.vertical_centered(|ui| {
                    ui.allocate_ui(egui::vec2(260.0, 260.0), |ui| {
                        egui::Grid::new("calc_grid")
                            .spacing([10.0, 10.0])
                            .min_col_width(55.0)
                            .show(ui, |ui| {
                                let rows = [
                                    ["7", "8", "9", "/"],
                                    ["4", "5", "6", "*"],
                                    ["1", "2", "3", "-"],
                                    ["C", "0", ".", "+"],
                                ];
                            
                                for row in rows {
                                    for &label in row.iter() {
                                        let color = match label {
                                            "+" | "-" | "*" | "/" => egui::Color32::from_rgb(255, 165, 0),
                                            "C" => egui::Color32::from_rgb(200, 50, 50),
                                            _ => ui.visuals().widgets.noninteractive.bg_fill,
                                        };
                                    
                                        let btn = egui::Button::new(
                                            egui::RichText::new(label).size(20.0).strong()
                                        )
                                        .fill(color)
                                        .min_size(egui::vec2(55.0, 55.0));
                                    
                                        if ui.add(btn).clicked() {
                                            match label {
                                                "C" => {
                                                    self.input.clear();
                                                    self.result = 0.0;
                                                }
                                                _ => self.input.push_str(label),
                                            }
                                        }
                                    }
                                    ui.end_row();
                                }
                            });
                    });
                });

                ui.add_space(15.0);

                // --- EQUALS BUTTON ---
                let equals_btn = egui::Button::new(egui::RichText::new("=").size(24.0).strong())
                    .fill(egui::Color32::from_rgb(70, 130, 180))
                    .min_size(egui::vec2(250.0, 50.0));

                if ui.add(equals_btn).clicked() || ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                    match eval_str(&self.input) {
                        Ok(val) => self.result = val,
                        Err(_) => self.result = f64::NAN,
                    }
                }
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 520.0]) // Adjusted height slightly for the new layout
            .with_resizable(false),
        ..Default::default()
    };
    
    let _ = eframe::run_native(
        "Rust Calculator",
        options,
        Box::new(|_cc| Box::<Calculator>::default()),
    );
}