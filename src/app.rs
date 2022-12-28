use std::{
    thread,
    time::{Duration, SystemTime},
};

use crate::colors::{GREEN, RED};
use crate::sounds::{finish_sound, start_sound};

/// The tomato timer data structure. We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TomatoTimer {
    time_per_round: f32,
    time_per_break: f32,
    start_time: SystemTime,
    session_count: i32,
    #[serde(skip)] // don't persist this field
    status: TimerStatus,
}

#[derive(Debug)]
enum TimerStatus {
    Running,
    Break(SystemTime), // time when break startedz
    Stopped,
}

impl Default for TomatoTimer {
    fn default() -> Self {
        Self {
            time_per_round: 25.0,
            time_per_break: 5.0,
            session_count: 0,
            start_time: SystemTime::now(),
            status: TimerStatus::Stopped,
        }
    }
}

impl TomatoTimer {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_pixels_per_point(3.0);

        // custom styling
        let mut style: egui::Style = (*cc.egui_ctx.style()).clone();

        style.visuals.override_text_color = Some(egui::Color32::WHITE);

        cc.egui_ctx.set_style(style);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TomatoTimer {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            time_per_round,
            time_per_break,
            start_time,
            session_count,
            status,
        } = self;

        ctx.request_repaint_after(Duration::from_millis(10)); // request a repaint every 10 ms

        if let TimerStatus::Running = status {
            let mut style: egui::Style = (*ctx.style()).clone();
            style.visuals.panel_fill = RED;
            ctx.set_style(style);
        } else {
            let mut style: egui::Style = (*ctx.style()).clone();
            style.visuals.panel_fill = GREEN;
            ctx.set_style(style);
        }

        // main window
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Task Timer");

            ui.label(format!("Deep Work Session Count: {}", session_count));

            ui.separator();

            ui.add(egui::Slider::new(time_per_round, 1.0..=60.0).text("Minutes per round"));
            ui.add(egui::Slider::new(time_per_break, 1.0..=60.0).text("Minutes per break"));

            let current_time = SystemTime::now();

            let elapsed_time = match status {
                TimerStatus::Running => current_time
                    .duration_since(*start_time)
                    .unwrap()
                    .as_secs_f32(),
                TimerStatus::Break(break_start) => current_time
                    .duration_since(*break_start)
                    .unwrap()
                    .as_secs_f32(),
                _ => 0.0,
            };

            let is_round_complete = match status {
                TimerStatus::Running => elapsed_time > (*time_per_round * 60.0),
                TimerStatus::Break(_) => elapsed_time > (*time_per_break * 60.0),
                _ => false,
            };

            let remaining_time = match status {
                TimerStatus::Running => (*time_per_round * 60.0) - elapsed_time,

                TimerStatus::Break(_) => (*time_per_break * 60.0) - elapsed_time,

                _ => 0.0,
            };

            let button_text = match status {
                TimerStatus::Running | TimerStatus::Break(_) => "Stop Round",
                _ => "Start Round",
            };

            // main timer logic and actions
            match status {
                // if we have no time remaining, show a notification, play a sound, and switch to break mode
                TimerStatus::Running => {
                    if is_round_complete {
                        notifica::notify("Time is up!", "Take a break").unwrap();

                        thread::spawn(|| {
                            finish_sound();
                        });

                        *session_count += 1;

                        *status = TimerStatus::Break(current_time);
                    }
                }

                // if we have no time remaining in the break, switch to work mode
                TimerStatus::Break(_) => {
                    if is_round_complete {
                        notifica::notify("Back to work!", "Start focusing again :)").unwrap();

                        thread::spawn(|| {
                            start_sound();
                        });

                        *status = TimerStatus::Running;
                    }
                }

                _ => {} // do nothing if we're stopped
            }

            // time remaining label
            ui.heading(match remaining_time as u32 {
                60..=u32::MAX => format!(
                    "{} Minutes left in round",
                    (Duration::from_secs_f32(remaining_time).as_secs() / 60) + 1
                ),
                1..=59 => format!(
                    "{} Seconds left in round",
                    Duration::from_secs_f32(remaining_time).as_secs()
                ),
                _ => format!("Time is up!"),
            });

            // start / stop button

            ui.horizontal(|ui| {
                if ui.button(button_text).clicked() {
                    match status {
                        TimerStatus::Stopped => {
                            *start_time = current_time;
                            *status = TimerStatus::Running;
                            thread::spawn(|| {
                                start_sound();
                            });
                        }
                        _ => {
                            *start_time = current_time;
                            *status = TimerStatus::Stopped;
                        }
                    }
                }
                if ui.button("Reset Session Count").clicked() {
                    *session_count = 0;
                }
            });
        });
    }
}
