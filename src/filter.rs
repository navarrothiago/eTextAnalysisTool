#[derive(PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]

/// Shows off a table with dynamic layout
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FilterTable {
    striped: bool,
    resizable: bool,
    clickable: bool,
    scroll_to_row: Option<usize>,
    selection: std::collections::HashSet<usize>,
    checked: bool,
    reversed: bool,
}

impl Default for FilterTable {
    fn default() -> Self {
        Self {
            striped: true,
            resizable: true,
            clickable: true,
            scroll_to_row: None,
            selection: Default::default(),
            checked: false,
            reversed: false,
        }
    }
}

impl crate::Demo for FilterTable {
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

const NUM_MANUAL_ROWS: usize = 20;

impl crate::View for FilterTable {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let mut reset = false;

        // let max_width = ui.available_width();
        egui::ScrollArea::horizontal()
            .id_salt("filter_scroll")
            // .max_width(max_width)
            .show(ui, |ui| {
                self.table_ui(ui, reset);
            });
    }
}

impl FilterTable {
    fn table_ui(&mut self, ui: &mut egui::Ui, reset: bool) {
        use egui_extras::{Column, TableBuilder};

        let available_height = ui.available_height();
        let mut table = TableBuilder::new(ui)
            .id_salt("filter_table")
            .striped(self.striped)
            .resizable(self.resizable)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::remainder())
            .column(Column::remainder())
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
                            ui.strong("ID");
                        },
                        |ui| {
                            self.reversed ^=
                                ui.button(if self.reversed { "⬆" } else { "⬇" }).clicked();
                        },
                    );
                });
                header.col(|ui| {
                    ui.strong("Modifiers");
                });
                header.col(|ui| {
                    ui.strong("Pattern");
                });
                header.col(|ui| {
                    ui.strong("Description");
                });
                header.col(|ui| {
                    ui.strong("Hits");
                });
            })
            .body(|mut body| {
                for row_index in 0..NUM_MANUAL_ROWS {
                    let row_index = if self.reversed {
                        NUM_MANUAL_ROWS - 1 - row_index
                    } else {
                        row_index
                    };

                    let is_thick = thick_row(row_index);
                    let row_height = if is_thick { 30.0 } else { 18.0 };
                    body.row(row_height, |mut row| {
                        row.set_selected(self.selection.contains(&row_index));

                        row.col(|ui| {
                            ui.checkbox(&mut self.checked, row_index.to_string());
                        });
                        row.col(|ui| {
                            ui.label("".to_string());
                        });
                        row.col(|ui| {
                            ui.label(format!("pattern {}", row_index));
                        });
                        row.col(|ui| {
                            ui.label(format!("description {}", row_index));
                        });
                        row.col(|ui| {
                            ui.label(format!("{}", row_index));
                        });
                        self.toggle_row_selection(row_index, &row.response());
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
