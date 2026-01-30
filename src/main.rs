// src/main.rs
mod data;
use data::{get_all_100_questions, calculate_result, Question, Archetype};
use eframe::egui;

// Android entry point
#[cfg(target_os = "android")]
use eframe::android_activity::AndroidApp;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Nexus Archetype",
        options,
        Box::new(|_cc| Box::new(NexusApp::default())),
    ).unwrap();
}

// Desktop entry point
#[cfg(not(target_os = "android"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Nexus Archetype",
        options,
        Box::new(|_cc| Ok(Box::new(NexusApp::default()))),
    )
}

struct NexusApp {
    state: AppState,
    questions: Vec<Question>,
    answers: Vec<u8>,
    current_index: usize,
    result: Option<Archetype>,
}

enum AppState {
    Welcome,
    Quiz,
    Result,
}

impl Default for NexusApp {
    fn default() -> Self {
        Self {
            state: AppState::Welcome,
            questions: get_all_100_questions(),
            answers: Vec::new(),
            current_index: 0,
            result: None,
        }
    }
}

impl eframe::App for NexusApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let central_panel = egui::CentralPanel::default();
        
        central_panel.show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                match self.state {
                    AppState::Welcome => {
                        ui.add_space(50.0);
                        ui.heading("The Nexus Archetype System");
                        ui.add_space(20.0);
                        ui.label("100 Questions. 50 Archetypes.");
                        ui.label("Discover your psychological profile.");
                        ui.add_space(50.0);
                        if ui.button("Begin Assessment").clicked() {
                            self.state = AppState::Quiz;
                        }
                    }
                    AppState::Quiz => {
                        let progress = self.current_index as f32 / self.questions.len() as f32;
                        ui.add(egui::ProgressBar::new(progress).show_percentage());
                        ui.add_space(20.0);

                        if self.current_index < self.questions.len() {
                            let q = &self.questions[self.current_index];
                            ui.label(format!("Question {} / {}", self.current_index + 1, self.questions.len()));
                            ui.add_space(10.0);
                            ui.heading(q.text);
                            ui.add_space(30.0);

                            // The 1-5 Scale
                            let buttons = [
                                (1, "Strongly Agree", egui::Color32::from_rgb(100, 255, 100)),
                                (2, "Agree", egui::Color32::from_rgb(150, 255, 150)),
                                (3, "Neutral", egui::Color32::GRAY),
                                (4, "Disagree", egui::Color32::from_rgb(255, 150, 150)),
                                (5, "Strongly Disagree", egui::Color32::from_rgb(255, 100, 100)),
                            ];

                            for (val, text, color) in buttons {
                                if ui.add(egui::Button::new(text).min_size(egui::vec2(200.0, 40.0)).fill(color)).clicked() {
                                    self.answers.push(val);
                                    self.current_index += 1;
                                }
                                ui.add_space(5.0);
                            }
                        } else {
                            // Quiz finished
                            self.result = Some(calculate_result(&self.answers));
                            self.state = AppState::Result;
                        }
                    }
                    AppState::Result => {
                        if let Some(res) = &self.result {
                            ui.add_space(30.0);
                            ui.heading("Your Archetype Is");
                            ui.add_space(10.0);
                            ui.heading(egui::RichText::new(res.name).size(40.0).color(egui::Color32::LIGHT_BLUE));
                            ui.add_space(20.0);
                            ui.label(egui::RichText::new(res.description).size(18.0));
                            ui.add_space(40.0);
                            
                            ui.label("Stats:");
                            ui.label(format!("Empathy Score: {:.1}", res.e_score));
                            ui.label(format!("Order Score: {:.1}", res.o_score));
                            ui.label(format!("Proactivity Score: {:.1}", res.p_score));
                            
                            ui.add_space(40.0);
                            if ui.button("Retake Test").clicked() {
                                self.answers.clear();
                                self.current_index = 0;
                                self.result = None;
                                self.state = AppState::Welcome;
                            }
                        }
                    }
                }
            });
        });
    }
}
