use std::{
    thread,
    time::{Duration, SystemTime},
};

use crate::sounds::{finish_sound, start_sound};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct TemplateApp {
    #[serde(skip)]
    timer: Timer,
}

struct Timer {
    time_per_round: f32,
    time_per_break: f32,
    start_time: SystemTime,
    status: Option<TimerStatus>,
}

#[derive(Debug)]
enum TimerStatus {
    Running,
    Break(SystemTime), // time when break started
    Stopped,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            timer: Timer {
                time_per_round: 25.0,
                time_per_break: 5.0,
                start_time: SystemTime::now(),
                status: Some(TimerStatus::Stopped),
            },
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_pixels_per_point(3.0);

        // custom styling
        let mut style: egui::Style = (*cc.egui_ctx.style()).clone();
        // style.spacing.item_spacing = egui::vec2(10.0, 20.0);
        style.visuals.panel_fill = egui::Color32::from_rgb(5, 130, 202);

        style.visuals.widgets = egui::style::Widgets {
            noninteractive: egui::style::WidgetVisuals {
                bg_fill: egui::Color32::from_rgb(0, 100, 148),
                bg_stroke: egui::Stroke::new(1.0, egui::Color32::BLACK),
                rounding: egui::Rounding::default(),
                fg_stroke: egui::Stroke::new(3.0, egui::Color32::BLACK),
                expansion: 0.0,
            },
            inactive: egui::style::WidgetVisuals {
                bg_fill: egui::Color32::from_rgb(0, 53, 84),
                bg_stroke: egui::Stroke::new(1.0, egui::Color32::BLACK),
                rounding: egui::Rounding::default(),
                fg_stroke: egui::Stroke::new(1.0, egui::Color32::WHITE),
                expansion: 0.0,
            },
            ..Default::default()
        };

        //212, 193, 236
        cc.egui_ctx.set_style(style);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            timer:
                Timer {
                    time_per_round,
                    time_per_break,
                    start_time,
                    status,
                },
        } = self;

        ctx.request_repaint_after(Duration::from_millis(10)); // request a repaint every second

        // main window
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Task Timer");

            ui.separator();

            ui.add(egui::Slider::new(time_per_round, 0.0..=60.0).text("Minutes per round"));
            ui.add(egui::Slider::new(time_per_break, 0.0..=60.0).text("Minutes per break"));

            let current_time = SystemTime::now();

            let elapsed_time = match status {
                Some(TimerStatus::Running) => current_time
                    .duration_since(*start_time)
                    .unwrap()
                    .as_secs_f32(),
                Some(TimerStatus::Break(start)) => {
                    current_time.duration_since(*start).unwrap().as_secs_f32()
                }
                _ => 0.0,
            };

            let remaining_time = match status {
                Some(TimerStatus::Running) => (*time_per_round * 60.0) - elapsed_time,

                Some(TimerStatus::Break(_)) => (*time_per_break * 60.0) - elapsed_time,

                _ => 0.0,
            };

            let button_text = match status {
                Some(TimerStatus::Running | TimerStatus::Break(_)) => "Stop Round",
                _ => "Start Round",
            };

            // main timer logic and actions
            // if we're running, show the progress bar
            match status {
                Some(TimerStatus::Running) => {
                    if remaining_time > 0.0 {
                        ui.horizontal(|ui| {
                            // ui.add(egui::ProgressBar::new(remaining_time / *time_per_round));
                            ui.add(egui::ProgressBar::new(
                                remaining_time / (*time_per_round * 60.0),
                            ));
                            ui.label(format!(
                                "{}",
                                Duration::from_secs_f32(remaining_time).as_secs()
                            ));
                        });
                    } else {
                        notifica::notify("Time is up!", "Take a break").unwrap();
                        thread::spawn(|| {
                            finish_sound();
                        });
                        *status = Some(TimerStatus::Break(current_time)); // reset timer so we don't spam notifications
                        ui.add(egui::ProgressBar::new(0.0));
                    }
                }
                Some(TimerStatus::Break(_)) => {
                    if remaining_time > 0.0 {
                        ui.horizontal(|ui| {
                            ui.add(egui::ProgressBar::new(
                                remaining_time / (*time_per_break * 60.0),
                            ));
                            ui.label(format!(
                                "Seconds left in break: {}",
                                Duration::from_secs_f32(remaining_time).as_secs()
                            ));
                        });
                    } else {
                        notifica::notify("Back to work!", "Start focusing again :)").unwrap();
                        thread::spawn(|| {
                            start_sound();
                        });
                        *status = Some(TimerStatus::Running); // reset timer so we don't spam notifications
                        ui.add(egui::ProgressBar::new(0.0));
                    }
                }
                _ => {}
            }

            // time remaining label
            ui.heading(
                //rustfmt::skip
                if remaining_time > 60.0 {
                    format!(
                        "{} Minutes left in round",
                        (Duration::from_secs_f32(remaining_time).as_secs() / 60) + 1
                    )
                } else if remaining_time > 0.0 {
                    format!(
                        "{} Seconds left in round",
                        Duration::from_secs_f32(remaining_time).as_secs()
                    )
                } else {
                    "Time is up!".to_owned()
                },
            );

            // start / stop button
            if ui.button(button_text).clicked() {
                match status {
                    Some(TimerStatus::Stopped) => {
                        if time_per_round == &0.0 {
                            return;
                        }
                        *start_time = current_time;
                        *status = Some(TimerStatus::Running);
                        thread::spawn(|| {
                            start_sound();
                        });
                    }
                    _ => {
                        *status = Some(TimerStatus::Stopped);
                    }
                }
            }
            ui.heading(format!("{:?}", status));
        });
    }
}
