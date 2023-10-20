use egui::{Context, Id, Label, RichText, Sense, Window};

/// About
#[derive(Debug, Default)]
pub(in crate::app) struct About {
    pub(in crate::app) open: bool,
}

impl About {
    pub(in crate::app) fn window(&mut self, ctx: &Context) {
        Window::new("ℹ About").open(&mut self.open).show(ctx, |ui| {
            // let color = ui.visuals().text_color();
            // let mut text = LayoutJob::default();
            // text.append(
            //     &format!(
            //         "UTCA {version}\n\
            //         Ultimate TAG Calculation Application\n\
            //         © 2023 Giorgi Kazakov & Roman Sidorov"
            //     ),
            //     0.0,
            //     TextFormat {
            //         color,
            //         ..Default::default()
            //     },
            // );
            // ctx.frame_nr()

            ui.vertical_centered(|ui| {
                let version = env!("CARGO_PKG_VERSION");
                ui.label(&format!("UTCA {version}"));
                ui.label("Ultimate TAG Calculation Application");
                ui.label("© 2023");
                ui.add(Label::new("Giorgi Kazakov").sense(Sense::click()));
                let id = Id::new("counter");
                let counter = ui.data_mut(|data| data.get_temp::<usize>(id).unwrap_or_default());
                let mut response = ui.add(Label::new("Roman Sidorov").sense(Sense::click()));
                if counter > 42 {
                    response = response.on_hover_text("♥ лучший котик в мире");
                }
                if response.clicked() {
                    ui.data_mut(|data| data.insert_temp(id, counter + 1));
                } else if ui.input(|input| input.pointer.any_click()) {
                    ui.data_mut(|data| data.remove::<usize>(id));
                }
                ui.separator();
                ui.collapsing(RichText::new("Links").heading(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("🌐 web:");
                        ui.hyperlink_to(
                            "https://ippras.github.io/utca",
                            "https://ippras.github.io/utca",
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label(" github:");
                        ui.hyperlink_to(
                            "https://github.com/ippras/utca",
                            "https://github.com/ippras/utca",
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("⚠ issues:");
                        ui.hyperlink_to(
                            "https://github.com/ippras/utca/issues",
                            "https://github.com/ippras/utca/issues",
                        );
                    });
                });
                ui.collapsing(RichText::new("Dedications").heading(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Giorgi Kazakov:");
                        ui.label("Моим родителям, Тане и Володе, посвящается.");
                    });
                });
            });
        });
    }
}
