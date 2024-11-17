use crate::{FilterTable, LogTable, View};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TextAnalysisTool {
    #[serde(skip)] // This how you opt-out of serialization of a field
    log_table: LogTable,
    #[serde(skip)] // This how you opt-out of serialization of a field
    filter_table: FilterTable,
}

impl Default for TextAnalysisTool {
    fn default() -> Self {
        Self {
            log_table: Default::default(),
            filter_table: Default::default(),
        }
    }
}

impl TextAnalysisTool {
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

impl eframe::App for TextAnalysisTool {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
                // ui.allocate_ui_with_layout(
                //     egui::Vec2::new(ui.available_width(), ui.available_height()),
                //     egui::Layout::right_to_left(egui::Align::Center),
                //     |ui| {
                //         egui::widgets::global_theme_preference_switch(ui);
                //     });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.ctx().options_mut(|opt| opt.warn_on_id_clash = false);

            let available_height = ui.available_height();

            ui.allocate_ui_with_layout(
                egui::Vec2::new(ui.available_width(), available_height * (2.0 / 3.0)),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    self.log_table.ui(ui);
                },
            );

            ui.add_space(10.0); // Optional: Add some spacing between the sections

            ui.allocate_ui_with_layout(
                egui::Vec2::new(ui.available_width(), available_height * (1.0 / 3.0)),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    self.filter_table.ui(ui);
                },
            );
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            powered_by_egui_and_eframe(ui);
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

pub fn lines_text() -> Vec<&'static str> {
    TEXT.lines().collect()
}

// create random text const
const TEXT: &str = include_str!("../assets/text.txt");
