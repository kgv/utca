use egui::{
    CollapsingHeader, Context, Grid, Id, Label, Response, RichText, Sense, Ui, Widget, Window,
};
use ehttp::{fetch, Headers, Request, Response as EhttpResponse};
use poll_promise::Promise;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;
use tracing::{error, info};
use url::Url;

/// `github.com tree` renders a nested list of debugger values.
pub struct Github {
    pub open: bool,
    pub url: String,
    promise: Promise<Vec<Entry>>,
}

impl Default for Github {
    fn default() -> Self {
        Self::new("https://api.github.com/repos/ippras/utca/contents/configs")
    }
}

impl Github {
    pub fn new(url: impl ToString) -> Self {
        let url = url.to_string();
        let promise = load(&url);
        Self {
            open: false,
            url,
            promise,
        }
    }

    pub fn window(&mut self, ctx: &Context) {
        Window::new("📥 Load").open(&mut self.open).show(ctx, |ui| {
            if let Some(entries) = self.promise.ready() {
                for entry in entries {
                    match entry.r#type {
                        Type::Dir => {
                            ui.collapsing(&entry.name, |ui| {
                                // load(self.url)
                            });
                        }
                        Type::File => {
                            ui.horizontal(|ui| {
                                if ui.button("📥").clicked() {
                                    if let Some(url) = &entry.download_url {
                                        // let promise: Promise<String> = load(url);
                                    }
                                }
                                ui.label(&entry.name);
                            });
                        }
                    }
                }
            } else {
                ui.spinner();
            }
        });
    }
}

// impl Widget for &mut GithubTree {
//     fn ui(self, ui: &mut Ui) -> Response {
//         if let Some(entries) = self.promise.ready() {
//             for entry in entries {
//                 match entry.r#type {
//                     Type::Dir => {
//                         ui.collapsing(&entry.name, |ui| {
//                             // load(self.url)
//                         });
//                     }
//                     Type::File => {
//                         ui.horizontal(|ui| {
//                             ui.label(&entry.name);
//                             if ui.button("📥").clicked() {
//                                 if let Some(url) = &entry.download_url {
//                                     let promise: Promise<String> = load(url);
//                                 }
//                             }
//                         });
//                     }
//                 }
//             }
//         }
//         ui.spinner()
//         // Grid::new(ui.next_auto_id())
//         //     .num_columns(3)
//         //     .striped(true)
//         //     .show(ui, |ui| {
//         //         for value in self.values {
//         //             if value.children().count() > 0 {
//         //                 CollapsingHeader::new(value.name().expect("name should be present"))
//         //                     .id_source(ui.next_auto_id())
//         //                     .show(ui, |ui| {
//         //                         ui.add(VariableList::new(value.children()));
//         //                     });
//         //             } else {
//         //                 ui.label(value.name().unwrap_or_default());
//         //                 ui.label(value.display_type_name().unwrap_or_default());
//         //                 ui.label(value.value().unwrap_or_default());
//         //             }
//         //             ui.end_row();
//         //         }
//         //     })
//         //     .response
//     }
// }

fn load(url: impl ToString) -> Promise<Vec<Entry>> {
    let request = Request {
        headers: Headers::new(&[
            ("Accept", "application/vnd.github+json"),
            (
                "Authorization",
                "Bearer ghp_",
            ),
            ("X-GitHub-Api-Version", "2022-11-28"),
        ]),
        ..Request::get(url)
    };
    let (sender, promise) = Promise::new();
    fetch(request, move |response| match response {
        Ok(response) => match response.json::<Vec<Entry>>() {
            Ok(mut entries) => {
                println!("entries: {entries:#?}");
                entries.sort_by_key(|entry| entry.r#type);
                sender.send(entries);
            }
            Err(error) => {
                error!(%error);
                info!("Status code: {}", response.status);
                sender.send(Default::default());
            }
        },
        Err(error) => {
            error!(%error);
            sender.send(Default::default());
        }
    });
    promise
}

// fn done(response: ehttp::Result<ehttp::Response>) -> anyhow::Result<Vec<Entry>> {
//     let response = response?;
//     info!("Status code: {}", response.status);
//     Ok(response.json()?)
// }

// #[derive(Clone, Debug, Error)]
// enum Error {
//     #[error("transparent")]
//     Response(String),
//     #[error(transparent)]
//     Json(#[from] serde_json::Error),
// }

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entry {
    pub name: String,
    pub size: usize,
    pub download_url: Option<Url>,
    pub r#type: Type,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Dir,
    File,
}
