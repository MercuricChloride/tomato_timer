use egui::Color32;

pub const GREEN: Color32 = egui::Color32::from_rgb(64, 145, 108);
pub const RED: Color32 = egui::Color32::from_rgb(158, 42, 43);

// SAMPLE CODE FOR CHANGING WIDGET COLORS
// style.visuals.widgets = egui::style::Widgets {
//     noninteractive: egui::style::WidgetVisuals {
//         bg_fill: egui::Color32::DARK_GREEN,
//         bg_stroke: egui::Stroke::new(1.0, egui::Color32::BLACK),
//         rounding: egui::Rounding::default(),
//         fg_stroke: egui::Stroke::new(3.0, egui::Color32::BLACK),
//         expansion: 0.0,
//     },
//     inactive: egui::style::WidgetVisuals {
//         bg_fill: egui::Color32::DARK_GREEN,
//         bg_stroke: egui::Stroke::new(1.0, egui::Color32::BLACK),
//         rounding: egui::Rounding::default(),
//         fg_stroke: egui::Stroke::new(1.0, egui::Color32::WHITE),
//         expansion: 0.0,
//     },
//     ..Default::default()
// };
