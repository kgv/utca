use egui::{Context, Id, Label, Sense, Window};

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

            let version = env!("CARGO_PKG_VERSION");
            ui.vertical_centered(|ui| {
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
                ui.hyperlink_to("web", "https://ippras.github.io/utca/");
                ui.hyperlink_to("github", "https://github.com/ippras/utca");
                ui.hyperlink_to("issues", "https://github.com/ippras/utca/issues");
            });
        });
    }
}
