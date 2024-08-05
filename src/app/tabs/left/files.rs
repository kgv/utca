use std::sync::Arc;

use crate::app::{context::Context, view::View};
use egui::{Id, Label, Ui};
use egui_dnd::dnd;
use ehttp::{fetch, Headers, Request};
use poll_promise::Promise;
use serde::{Deserialize, Serialize};
use tracing::error;
use url::Url;

/// Files
// #[derive(Debug)]
pub(super) struct Files<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Files<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl View for Files<'_> {
    fn view(mut self, ui: &mut Ui) {
        let Self { context, .. } = self;
        let mut remove = None;
        dnd(ui, Id::new("dnd").with("files")).show_vec(
            &mut context.state.entries,
            |ui, item, handle, state| {
                ui.horizontal(|ui| {
                    handle.ui(ui, |ui| {
                        let _ = ui.button(if state.dragged { "👊" } else { "✋" });
                    });
                    ui.radio_value(&mut context.state.index, state.index, "");
                    ui.add(Label::new(&item.meta.name).truncate(true));
                    if ui.button("🗑").clicked() {
                        remove = Some(state.index);
                    }
                });
            },
        );
        if let Some(index) = remove {
            context.state.entries.remove(index);
            if index <= context.state.index {
                context.state.index = context.state.index.saturating_sub(1);
            }
            if context.state.entries.is_empty() {
                context.state.entries.push(Default::default());
            }
        }
        ui.separator();
        // let promise: Option<Option<true>> = ui.data(|data| data.get_temp(Id::new("configs")));
        // if let Some(promise) = &promise {
        // } else {
        //     let request = Request {
        //         headers: Headers::new(&[
        //             ("Accept", "application/vnd.github+json"),
        //             (
        //                 "Authorization",
        //                 "Bearer ghp_",
        //             ),
        //             ("X-GitHub-Api-Version", "2022-11-28"),
        //         ]),
        //         ..Request::get("https://api.github.com/repos/ippras/utca/contents/configs")
        //     };
        //     // let (sender, promise) = Promise::new();
        //     let promise = Promise::spawn_local(async {
        //         let response = fetch_async(request).await?;
        //         Ok(response.json::<Vec<Entry>>()?)
        //         // , move |response| {
        //         //     // ctx.forget_image(&prev_url);
        //         //     // ctx.request_repaint(); // wake up UI thread
        //         //     // match response.map(|response| response.json::<Vec<Entry>>()) {
        //         //     //     Ok(Ok(entries)) => {
        //         //     //         println!("entries: {entries:#?}");
        //         //     //         // sender.send(entries);
        //         //     //     }
        //         //     //     Ok(Err(error)) => {
        //         //     //         error!(%error)
        //         //     //     }
        //         //     //     Err(error) => {
        //         //     //         error!(%error)
        //         //     //     }
        //         //     // }
        //         //     // Some(())
        //         // });
        //     });
        //     // fetch(request, move |response| {
        //     //     // ctx.forget_image(&prev_url);
        //     //     // ctx.request_repaint(); // wake up UI thread
        //     //     match response.map(|response| response.json::<Vec<Entry>>()) {
        //     //         Ok(Ok(entries)) => {
        //     //             println!("entries: {entries:#?}");
        //     //             sender.send(entries);
        //     //         }
        //     //         Ok(Err(error)) => {
        //     //             error!(%error)
        //     //         }
        //     //         Err(error) => {
        //     //             error!(%error)
        //     //         }
        //     //     }
        //     // });
        //     ui.data(|data| data.insert_temp(Id::new("configs"), promise));
        // }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Entry {
    pub name: String,
    pub size: usize,
    pub download_url: Option<Url>,
    pub r#type: String,
}
