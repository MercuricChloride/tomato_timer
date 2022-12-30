use std::{
    thread,
    time::{Duration, SystemTime},
};

use crate::{colors::get_color_for_timer_status, display_time_remaining, get_remaining_time};
use crate::{get_is_round_complete, handle_round_complete, sounds::start_sound};

/// The tomato timer data structure. We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TomatoTimer {
    pub time_per_round: f32,
    pub time_per_break: f32,
    pub session_count: i32,
    #[serde(skip)] // don't persist this field
    pub status: TimerStatus,
}

#[derive(Debug)]
pub enum TimerStatus {
    Running(SystemTime), // time when timer started
    Break(SystemTime),   // time when break started
    Stopped,
}

impl Default for TomatoTimer {
    fn default() -> Self {
        Self {
            time_per_round: 25.0,
            time_per_break: 5.0,
            session_count: 0,
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
            session_count,
            status,
        } = self;

        ctx.request_repaint_after(Duration::from_millis(10)); // request a repaint every 10 ms

        // change the background color based on the timer status
        let mut style: egui::Style = (*ctx.style()).clone();
        style.visuals.panel_fill = get_color_for_timer_status(status);
        ctx.set_style(style);

        // main window
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tomato Timer 🍅⏰");

            ui.label(format!("Deep Work Count: {}", session_count));

            ui.separator();

            ui.add(egui::Slider::new(time_per_round, 1.0..=60.0).text("Minutes per round"));
            ui.add(egui::Slider::new(time_per_break, 1.0..=60.0).text("Minutes per break"));

            let current_time = SystemTime::now();

            let remaining_time = get_remaining_time(status, time_per_round, time_per_break);

            let is_round_complete = get_is_round_complete(status, time_per_round, time_per_break);

            let button_text = match status {
                TimerStatus::Running(_) | TimerStatus::Break(_) => "Stop Round",
                _ => "Start Round",
            };

            // function to handle the round timer logic
            handle_round_complete(status, is_round_complete);

            // main timer logic and actions
            ui.heading(display_time_remaining(remaining_time));

            // start / stop button
            ui.horizontal(|ui| {
                if ui.button(button_text).clicked() {
                    if let TimerStatus::Stopped = status {
                        *status = TimerStatus::Running(current_time);
                        thread::spawn(|| {
                            start_sound();
                        });
                    } else {
                        *status = TimerStatus::Stopped;
                    }
                }
                if ui.button("Reset Session Count").clicked() {
                    *session_count = 0;
                }
            });
        });
    }
}
