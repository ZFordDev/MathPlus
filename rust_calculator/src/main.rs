// Hide the console window on Windows builds.
// This makes the app feel like a normal GUI program.
#![windows_subsystem = "windows"]

use eframe::egui;
use eframe::{App, Frame};
use meval::eval_str;

// The main calculator state.
// `input` holds the expression the user is typing.
// `result` stores the last evaluated value.
struct Calculator {
    input: String,
    result: f64,
}

// Provide a clean default state when the app starts.
impl Default for Calculator {
    fn default() -> Self {
        Self {
            input: String::new(),
            result: 0.0,
        }
    }
}

// The main UI logic for the calculator.
// This runs every frame and draws the interface.
impl App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(5.0);

                // Simple title
                ui.label(
                    egui::RichText::new("MATH+")
                        .strong()
                        .color(egui::Color32::GRAY),
                );

                // -------------------------------
                // Display area (input + result)
                // -------------------------------
                // `allocate_ui` gives this section a fixed size so the keypad
                // doesn't move around when numbers get long.
                ui.allocate_ui(egui::vec2(260.0, 100.0), |ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        ui.set_height(100.0);
                        ui.set_width(260.0);

                        ui.vertical(|ui| {
                            ui.add_space(10.0);

                            // Show the current input (right-aligned)
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.add_space(10.0);
                                    ui.label(
                                        egui::RichText::new(&self.input)
                                            .size(32.0)
                                            .monospace(),
                                    );
                                },
                            );

                            // Show the result (right-aligned)
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.add_space(10.0);

                                    // If evaluation failed, show "Error"
                                    let res_text = if self.result.is_nan() {
                                        "Error".to_string()
                                    } else {
                                        format!("= {}", self.result)
                                    };

                                    ui.label(
                                        egui::RichText::new(res_text)
                                            .size(18.0)
                                            .color(egui::Color32::LIGHT_BLUE),
                                    );
                                },
                            );
                        });
                    });
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                // -------------------------------
                // Button grid (numbers + operators)
                // -------------------------------
                ui.vertical_centered(|ui| {
                    ui.allocate_ui(egui::vec2(260.0, 260.0), |ui| {
                        egui::Grid::new("calc_grid")
                            .spacing([10.0, 10.0])
                            .min_col_width(55.0)
                            .show(ui, |ui| {
                                // Layout for the keypad
                                let rows = [
                                    ["7", "8", "9", "/"],
                                    ["4", "5", "6", "*"],
                                    ["1", "2", "3", "-"],
                                    ["C", "0", ".", "+"],
                                ];

                                for row in rows {
                                    for &label in row.iter() {
                                        // Color-code special buttons
                                        let color = match label {
                                            "+" | "-" | "*" | "/" =>
                                                egui::Color32::from_rgb(255, 165, 0),
                                            "C" =>
                                                egui::Color32::from_rgb(200, 50, 50),
                                            _ =>
                                                ui.visuals().widgets.noninteractive.bg_fill,
                                        };

                                        // Build the button
                                        let btn = egui::Button::new(
                                            egui::RichText::new(label)
                                                .size(20.0)
                                                .strong(),
                                        )
                                        .fill(color)
                                        .min_size(egui::vec2(55.0, 55.0));

                                        // Handle button clicks
                                        if ui.add(btn).clicked() {
                                            match label {
                                                "C" => {
                                                    // Clear everything
                                                    self.input.clear();
                                                    self.result = 0.0;
                                                }
                                                _ => {
                                                    // Append the pressed character
                                                    self.input.push_str(label);
                                                }
                                            }
                                        }
                                    }
                                    ui.end_row();
                                }
                            });
                    });
                });

                ui.add_space(15.0);

                // -------------------------------
                // Equals button
                // -------------------------------
                let equals_btn = egui::Button::new(
                    egui::RichText::new("=")
                        .size(24.0)
                        .strong(),
                )
                .fill(egui::Color32::from_rgb(70, 130, 180))
                .min_size(egui::vec2(250.0, 50.0));

                // Evaluate when "=" is clicked or Enter is pressed
                if ui.add(equals_btn).clicked()
                    || ctx.input(|i| i.key_pressed(egui::Key::Enter))
                {
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
    // Basic window options for the app.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 520.0]) // Slightly taller for spacing
            .with_resizable(false),
        ..Default::default()
    };

    // Launch the app.
    let _ = eframe::run_native(
        "Rust Calculator",
        options,
        Box::new(|_cc| Box::<Calculator>::default()),
    );
}

// Thanks for checking out MathPlus! 
// If you're learning Rust or building your own project and need a hand, 
// feel free to reach out I'm always happy to help.