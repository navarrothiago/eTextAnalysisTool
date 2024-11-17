use egui::{TextStyle};

use crate::app;

#[derive(PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]

/// Shows off a table with dynamic layout
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct LogTable {
    striped: bool,
    resizable: bool,
    clickable: bool,
    scroll_to_row_slider: usize,
    scroll_to_row: Option<usize>,
    selection: std::collections::HashSet<usize>,
    reversed: bool,
}

impl Default for LogTable {
    fn default() -> Self {
        Self {
            striped: true,
            resizable: true,
            clickable: true,
            scroll_to_row_slider: 0,
            scroll_to_row: None,
            selection: Default::default(),
            reversed: false,

        }
    }
}

impl crate::Demo for LogTable {
    fn name(&self) -> &'static str {
        "☰ Table"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .default_width(400.0)
            .show(ctx, |ui| {
                use crate::View as _;
                self.ui(ui);
            });
    }
}

impl crate::View for LogTable {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let mut reset = false;

        ui.vertical(|ui| {
            {
                let max_rows = app::lines_text().len();

                let slider_response = ui.add(
                    egui::Slider::new(&mut self.scroll_to_row_slider, 0..=max_rows)
                        .logarithmic(true)
                        .text("Go to line"),
                );
                if slider_response.changed() {
                    self.scroll_to_row = Some(self.scroll_to_row_slider);
                }
            }
        });

        ui.separator();

        // Leave room for the source code link after the table demo:
        let body_text_size = TextStyle::Body.resolve(ui.style()).size;
        use egui_extras::{Size, StripBuilder};
        // For some reason, if I use the same logic in both LogTable and FilterTable, but with different id, the scroll area for both
        // will be the same. So, I have to wrapper with another element (StripBuilder).
        StripBuilder::new(ui)
            .size(Size::remainder().at_least(100.0)) // for the table
            .size(Size::exact(body_text_size)) // for the source code link
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal()
                        .id_salt("log_scroll")
                        .show(ui, |ui| {
                        self.table_ui(ui, reset);
                    });
                });
                strip.cell(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("".to_string())
                        // ui.add(crate::egui_github_link_file!());
                    });
                });
            });
    }
}

impl LogTable {
    fn table_ui(&mut self, ui: &mut egui::Ui, reset: bool) {
        use egui_extras::{Column, TableBuilder};

        let available_height = ui.available_height();
        let mut table = TableBuilder::new(ui)
            .id_salt("log_table")
            .striped(self.striped)
            .resizable(self.resizable)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::remainder())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);

        if self.clickable {
            table = table.sense(egui::Sense::click());
        }

        if let Some(row_index) = self.scroll_to_row.take() {
            table = table.scroll_to_row(row_index, None);
        }

        if reset {
            table.reset();
        }

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    egui::Sides::new().show(
                        ui,
                        |ui| {
                            ui.strong("Line");
                        },
                        |ui| {
                            self.reversed ^=
                                ui.button(if self.reversed { "⬆" } else { "⬇" }).clicked();
                        },
                    );
                });
                header.col(|ui| {
                    ui.strong("Content");
                });
            })
            .body(|mut body|{
                    for row_index in 1..app::lines_text().len(){
                        let row_index = if self.reversed {
                            app::lines_text().len() - 1 - row_index
                        } else {
                            row_index
                        };

                        let is_thick = thick_row(row_index);
                        let row_height: f32 = if is_thick { 30.0 } else { 18.0 };
                        body.row(row_height, |mut row| {
                            row.set_selected(self.selection.contains(&row_index));

                            row.col(|ui| {
                                ui.label(row_index.to_string());
                            });
                            row.col(|ui| {
                                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                                ui.label(app::lines_text()[row_index]);
                                // ui.label("Normal row");
                            });

                            self.toggle_row_selection(row_index, &row.response());
                        });
                    }
                });
    }

    fn toggle_row_selection(&mut self, row_index: usize, row_response: &egui::Response) {
        if row_response.clicked() {
            if self.selection.contains(&row_index) {
                self.selection.remove(&row_index);
            } else {
                self.selection.insert(row_index);
            }
        }
    }
}

fn thick_row(row_index: usize) -> bool {
    row_index % 6 == 0
}
