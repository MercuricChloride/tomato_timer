/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    #[serde(skip)]
    tasks: Vec<Task>,
    fullscreen: bool,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    #[serde(skip)]
    sort_by_importance: bool,
}

struct Task {
    label: String,
    importance: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            tasks: vec![],
            value: 1.0,
            fullscreen: false,
            sort_by_importance: false,
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
            label,
            value,
            tasks,
            fullscreen,
            sort_by_importance,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                ui.menu_button("View", |ui| {
                    if ui.button("Toggle Fullscreen").clicked() {
                        *fullscreen = !*fullscreen;
                        _frame.set_fullscreen(*fullscreen);
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Add a task");

            ui.text_edit_singleline(label);

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("Importance"));

            if ui.button("Add Task").clicked() {
                tasks.push(Task {
                    label: label.to_owned(),
                    importance: *value,
                });
                *label = "".to_owned();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Area::new("Left").show(&ctx, |ui| {
                ui.label("Left");
            });
            egui::Area::new("Right").show(&ctx, |ui| {
                ui.label("Right");
            });

            ui.separator();

            for task in tasks {
                ui.horizontal(|ui| {
                    ui.label(task.label.to_owned());
                    ui.label(task.importance.to_string());
                });
                ui.separator();
            }
        });
    }
}
