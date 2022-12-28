use std::{
    thread,
    time::{Duration, SystemTime},
};

use crate::colors::{green, red};
use crate::sounds::{finish_sound, start_sound};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // #[serde(skip)]
    // timer: Timer,
    time_per_round: f32,
    time_per_break: f32,
    start_time: SystemTime,
    total_time: u128, // total time logged in the app
    #[serde(skip)]
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
            time_per_round: 25.0,
            time_per_break: 5.0,
            total_time: 0,
            start_time: SystemTime::now(),
            status: Some(TimerStatus::Stopped),
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

        style.visuals.override_text_color = Some(egui::Color32::WHITE);
        // style.visuals.panel_fill = green;

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
            time_per_round,
            time_per_break,
            total_time,
            start_time,
            status,
        } = self;

        ctx.request_repaint_after(Duration::from_millis(10)); // request a repaint every 10 ms

        match status {
            Some(TimerStatus::Running) => {
                let mut style: egui::Style = (*ctx.style()).clone();
                style.visuals.panel_fill = red;
                ctx.set_style(style);
            }
            _ => {
                let mut style: egui::Style = (*ctx.style()).clone();
                style.visuals.panel_fill = green;
                ctx.set_style(style);
            }
        }

        // main window
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Task Timer");

            ui.separator();

            ui.add(egui::Slider::new(time_per_round, 1.0..=60.0).text("Minutes per round"));
            ui.add(egui::Slider::new(time_per_break, 1.0..=60.0).text("Minutes per break"));

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
            match status {
                // if we have time remaining in the round, show the timer
                // if we have no time remaining, show a notification, play a sound, and switch to break mode
                Some(TimerStatus::Running) => {
                    if remaining_time > 0.0 {
                        ui.horizontal(|ui| {
                            ui.add(egui::ProgressBar::new(
                                remaining_time / (*time_per_round * 60.0),
                            ));
                        });
                    } else {
                        notifica::notify("Time is up!", "Take a break").unwrap();

                        thread::spawn(|| {
                            finish_sound();
                        });

                        *total_time += elapsed_time as u128; // add the elapsed work time to the total working time

                        *status = Some(TimerStatus::Break(current_time));
                    }
                }

                // if we have time remaining in the break, show the timer
                // otherwise, switch to work mode
                Some(TimerStatus::Break(_)) => {
                    if remaining_time > 0.0 {
                        ui.horizontal(|ui| {
                            ui.add(egui::ProgressBar::new(
                                remaining_time / (*time_per_break * 60.0),
                            ));
                        });
                    } else {
                        notifica::notify("Back to work!", "Start focusing again :)").unwrap();

                        thread::spawn(|| {
                            start_sound();
                        });

                        *status = Some(TimerStatus::Running);
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
            if ui.button(button_text).clicked() {
                match status {
                    Some(TimerStatus::Stopped) => {
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

            ui.heading(&format!(
                "You have spent a total of {} minutes working, keep going!",
                Duration::from_secs(*total_time as u64).as_secs_f32() / 60.0
            ));
        });
    }
}
