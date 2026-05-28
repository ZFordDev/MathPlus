// Hide the console window on Windows builds.
// This makes the app feel like a normal GUI program.
#![windows_subsystem = "windows"]

use eframe::egui;
use eframe::{App, Frame, Theme};
use meval::eval_str;

mod history;
mod history_ui;
mod shortcuts;
use history::History;
use shortcuts::handle_keyboard_shortcuts;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

struct Calculator {
    input: String,
    result: f64,
    history: History,
    show_history: bool,
    copied_at: Option<std::time::Instant>,
    scientific_mode: bool,
    theme_mode: ThemeMode,
}

// Provide a clean default state when the app starts.
impl Default for Calculator {
    fn default() -> Self {
        Self {
            input: String::new(),
            result: 0.0,
            history: History::new(100), // Keep last 100 calculations
            show_history: false,
            copied_at: None,
            scientific_mode: false,
            theme_mode: ThemeMode::default(),
        }
    }
}

impl Calculator {
    fn is_dark_mode(&self, ctx: &egui::Context) -> bool {
        match self.theme_mode {
            ThemeMode::System => ctx.style().visuals.dark_mode,
            ThemeMode::Light => false,
            ThemeMode::Dark => true,
        }
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        if self.theme_mode != ThemeMode::System {
            let visuals = match self.theme_mode {
                ThemeMode::Light => Theme::Light.egui_visuals(),
                ThemeMode::Dark => Theme::Dark.egui_visuals(),
                ThemeMode::System => unreachable!(),
            };
            ctx.set_visuals(visuals);
        }
    }

    fn theme_color(
        &self,
        ctx: &egui::Context,
        dark: egui::Color32,
        light: egui::Color32,
    ) -> egui::Color32 {
        if self.is_dark_mode(ctx) { dark } else { light }
    }
}

// The main UI logic for the calculator.
// This runs every frame and draws the interface.
impl App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Scientific mode right panel
        if self.scientific_mode {
            egui::SidePanel::right("scientific_panel")
                .resizable(false)
                .default_width(220.0)
                .min_width(200.0)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new("Scientific")
                                .size(16.0)
                                .strong()
                                .color(egui::Color32::from_rgb(160, 157, 232)),
                        );
                        ui.separator();
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new("Functions coming soon...")
                                .size(13.0)
                                .color(egui::Color32::from_rgb(140, 140, 150)),
                        );
                    });
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("MATH+")
                            .strong()
                            .color(self.theme_color(
                                ctx,
                                egui::Color32::GRAY,
                                egui::Color32::DARK_GRAY,
                            )),
                    );

                    ui.add_space(16.0);
                    egui::ComboBox::from_label("Theme")
                        .selected_text(match self.theme_mode {
                            ThemeMode::System => "System",
                            ThemeMode::Light => "Light",
                            ThemeMode::Dark => "Dark",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.theme_mode, ThemeMode::System, "System");
                            ui.selectable_value(&mut self.theme_mode, ThemeMode::Light, "Light");
                            ui.selectable_value(&mut self.theme_mode, ThemeMode::Dark, "Dark");
                        });
                });

                self.apply_theme(ctx);

                // --- FIXED DISPLAY AREA ---
                // allocate_ui ensures this block doesn't grow and hide the keypad
                ui.allocate_ui(egui::vec2(260.0, 100.0), |ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        ui.set_height(95.0);
                        ui.set_width(260.0);

                        ui.vertical(|ui| {
                            ui.add_space(10.0);

                            // Current Input
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.add_space(10.0);
                                    ui.label(
                                        egui::RichText::new(&self.input)
                                            .size(32.0)
                                            .monospace()
                                            .color(self.theme_color(
                                                ctx,
                                                egui::Color32::from_rgb(232, 232, 238),
                                                egui::Color32::from_rgb(24, 24, 28),
                                            )),
                                    );
                                },
                            );

                            // Calculated Result
                            // Calculated Result
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.add_space(10.0);

                                    let res_text = if self.result.is_nan() {
                                        "Error".to_string()
                                    } else {
                                        format!("= {}", self.result)
                                    };

                                    // Show "Copied!" flash or the copy button
                                    let just_copied = self
                                        .copied_at
                                        .map(|t| t.elapsed().as_millis() < 600)
                                        .unwrap_or(false);

                                    if just_copied {
                                        ui.label(egui::RichText::new("Copied!").size(11.0).color(
                                            self.theme_color(
                                                ctx,
                                                egui::Color32::from_rgb(160, 157, 232),
                                                egui::Color32::from_rgb(70, 90, 170),
                                            ),
                                        ));
                                        ctx.request_repaint();
                                    } else {
                                        let copy_btn = egui::Button::new(
                                            egui::RichText::new("📋").size(13.0).color(
                                                self.theme_color(
                                                    ctx,
                                                    egui::Color32::from_rgb(110, 110, 120),
                                                    egui::Color32::from_rgb(90, 90, 100),
                                                ),
                                            ),
                                        )
                                        .fill(egui::Color32::TRANSPARENT)
                                        .stroke(egui::Stroke::NONE)
                                        .min_size(egui::vec2(18.0, 18.0));

                                        if ui.add(copy_btn).clicked() {
                                            ui.output_mut(|o| {
                                                o.copied_text = self.result.to_string()
                                            });
                                            self.copied_at = Some(std::time::Instant::now());
                                            play_windows_click_sound();
                                        }
                                    }

                                    ui.label(egui::RichText::new(&res_text).size(18.0).color(
                                        self.theme_color(
                                            ctx,
                                            egui::Color32::from_rgb(160, 157, 232),
                                            egui::Color32::from_rgb(70, 90, 170),
                                        ),
                                    ));
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
                                        let (fill, text_color) = match label {
                                            "+" | "-" | "*" | "/" => (
                                                self.theme_color(
                                                    ctx,
                                                    egui::Color32::from_rgb(37, 37, 53),
                                                    egui::Color32::from_rgb(205, 210, 250),
                                                ),
                                                self.theme_color(
                                                    ctx,
                                                    egui::Color32::from_rgb(160, 157, 232),
                                                    egui::Color32::from_rgb(35, 45, 75),
                                                ),
                                            ),
                                            "C" => (
                                                self.theme_color(
                                                    ctx,
                                                    egui::Color32::from_rgb(46, 28, 28),
                                                    egui::Color32::from_rgb(255, 230, 230),
                                                ),
                                                self.theme_color(
                                                    ctx,
                                                    egui::Color32::from_rgb(232, 128, 128),
                                                    egui::Color32::from_rgb(145, 35, 50),
                                                ),
                                            ),
                                            _ => (
                                                self.theme_color(
                                                    ctx,
                                                    egui::Color32::from_rgb(42, 42, 48),
                                                    egui::Color32::from_rgb(236, 236, 240),
                                                ),
                                                self.theme_color(
                                                    ctx,
                                                    egui::Color32::from_rgb(232, 232, 238),
                                                    egui::Color32::from_rgb(24, 24, 28),
                                                ),
                                            ),
                                        };

                                        let btn = egui::Button::new(
                                            egui::RichText::new(label)
                                                .size(20.0)
                                                .strong()
                                                .color(text_color),
                                        )
                                        .fill(fill)
                                        .rounding(egui::Rounding::same(10.0))
                                        .stroke(egui::Stroke::new(
                                            0.5,
                                            self.theme_color(
                                                ctx,
                                                egui::Color32::from_rgba_premultiplied(
                                                    255, 255, 255, 10,
                                                ),
                                                egui::Color32::from_rgba_premultiplied(0, 0, 0, 15),
                                            ),
                                        ))
                                        .min_size(egui::vec2(55.0, 55.0));

                                        if ui.add(btn).clicked() {
                                            match label {
                                                "C" => {
                                                    self.input.clear();
                                                    self.result = 0.0;
                                                }
                                                _ => {
                                                    self.input.push_str(label);
                                                }
                                            }
                                            play_windows_click_sound();
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
                let equals_btn =
                    egui::Button::new(egui::RichText::new("=").size(24.0).strong().color(
                        self.theme_color(
                            ctx,
                            egui::Color32::from_rgb(212, 210, 247),
                            egui::Color32::from_rgb(245, 245, 255),
                        ),
                    ))
                    .fill(self.theme_color(
                        ctx,
                        egui::Color32::from_rgb(37, 37, 53),
                        egui::Color32::from_rgb(70, 100, 170),
                    ))
                    .rounding(egui::Rounding::same(12.0))
                    .min_size(egui::vec2(250.0, 50.0));

                // Evaluate when the equals button is clicked.
                if ui.add(equals_btn).clicked() {
                    match eval_str(&self.input) {
                        Ok(val) => {
                            self.result = val;
                            self.history.add_calculation(self.input.clone(), val);
                        }
                        Err(_) => self.result = f64::NAN,
                    }
                    play_windows_click_sound();
                }

                ui.add_space(10.0);

                // --- HISTORY BUTTON ---
                let history_btn =
                    egui::Button::new(egui::RichText::new("History").size(18.0).strong())
                        .fill(self.theme_color(
                            ctx,
                            egui::Color32::from_rgb(100, 100, 100),
                            egui::Color32::from_rgb(210, 210, 210),
                        ))
                        .stroke(egui::Stroke::new(
                            0.5,
                            self.theme_color(
                                ctx,
                                egui::Color32::from_rgba_premultiplied(255, 255, 255, 10),
                                egui::Color32::from_rgba_premultiplied(0, 0, 0, 15),
                            ),
                        ))
                        .min_size(egui::vec2(250.0, 35.0));

                if ui.add(history_btn).clicked() {
                    self.show_history = true;
                    play_windows_click_sound();
                }

                ui.add_space(6.0);

                // --- SCIENTIFIC MODE TOGGLE ---
                let sci_label = if self.scientific_mode {
                    "Basic Mode"
                } else {
                    "Scientific Mode"
                };
                let sci_btn = egui::Button::new(egui::RichText::new(sci_label).size(18.0).strong())
                    .fill(if self.scientific_mode {
                        self.theme_color(
                            ctx,
                            egui::Color32::from_rgb(60, 80, 120),
                            egui::Color32::from_rgb(100, 130, 190),
                        )
                    } else {
                        self.theme_color(
                            ctx,
                            egui::Color32::from_rgb(100, 100, 100),
                            egui::Color32::from_rgb(210, 210, 210),
                        )
                    })
                    .stroke(egui::Stroke::new(
                        0.5,
                        self.theme_color(
                            ctx,
                            egui::Color32::from_rgba_premultiplied(255, 255, 255, 10),
                            egui::Color32::from_rgba_premultiplied(0, 0, 0, 15),
                        ),
                    ))
                    .min_size(egui::vec2(250.0, 35.0));

                if ui.add(sci_btn).clicked() {
                    self.scientific_mode = !self.scientific_mode;
                    play_windows_click_sound();
                }
            });
        });

        handle_keyboard_shortcuts(
            ctx,
            &mut self.input,
            &mut self.result,
            &mut self.history,
            &mut self.show_history,
            &mut self.copied_at,
        );

        // Show history window if requested
        if self.show_history {
            let action =
                crate::history_ui::show_history_window(ctx, &self.history, &mut self.show_history);
            match action {
                crate::history_ui::HistoryAction::Clear => {
                    self.history.clear();
                }
                _ => {}
            }
        }
    }
}

#[cfg(windows)]
fn play_windows_click_sound() {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows::core::PCWSTR;
    use windows::Win32::Media::Audio::{PlaySoundW, SND_ALIAS, SND_ASYNC, SND_NODEFAULT};

    let alias: Vec<u16> = OsStr::new("SystemAsterisk")
        .encode_wide()
        .chain(Some(0))
        .collect();

    unsafe {
        let _ = PlaySoundW(PCWSTR(alias.as_ptr()), None, SND_ALIAS | SND_ASYNC | SND_NODEFAULT);
    }
}

#[cfg(not(windows))]
fn play_windows_click_sound() {}

fn main() {
    // Basic window options for the app.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 580.0])
            .with_resizable(true),
        follow_system_theme: true,
        default_theme: Theme::Dark,
        ..Default::default()
    };

    // Launch the app.
    let _ = eframe::run_native(
        "MathPlus",
        options,
        Box::new(|_cc| Box::<Calculator>::default()),
    );
}

// Thanks for checking out MathPlus!
// If you're learning Rust or building your own project and need a hand,
// feel free to reach out I'm always happy to help.
