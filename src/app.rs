use std::time::{Duration, SystemTime};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    #[serde(skip)]
    time_per_round: f32,

    #[serde(skip)]
    finish_time: Option<SystemTime>,

    #[serde(skip)]
    round_started: bool,

    notified: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            time_per_round: 0.0,
            finish_time: None,
            round_started: false,
            notified: false,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

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
            finish_time,
            round_started,
            notified,
        } = self;

        _frame.set_window_size(egui::Vec2::new(300.0, 500.0));
        ctx.set_pixels_per_point(3.0);

        // main window
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Task Timer");

            ui.separator();

            ui.add(egui::Slider::new(time_per_round, 0.0..=60.0).text("Seconds per round"));

            let current_time = SystemTime::now();

            if ui
                .button(if *round_started {
                    "Stop Round"
                } else {
                    "Start Round"
                })
                .clicked()
            {
                *round_started = !*round_started;

                let round_time = Duration::from_secs_f32(*time_per_round);

                if *round_started {
                    *finish_time = Some(current_time + round_time);
                } else {
                    *finish_time = None;
                }
            }

            ui.heading(if let Some(finish_time) = finish_time {
                let remaining_time = finish_time.duration_since(current_time);
                if let Ok(remaining_time) = remaining_time {
                    format!("Time remaining: {}", remaining_time.as_secs())
                } else {
                    if !*notified {
                        *notified = true;
                        notifica::notify("Timer Up!", "Take a quick break :)").unwrap();
                    }
                    "Time is up!".to_string()
                }
            } else {
                "No Clock Started".to_string()
            });
        });
    }
}
