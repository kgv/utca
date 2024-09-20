//! `trunk serve --address=0.0.0.0`
//! - nix:  
//! `RUST_LOG=none,utca=trace cargo run`
//! - win:  
//! `$env:RUST_LOG="none,utca=trace"` `cargo run`
//!
//! `rustup target add wasm32-unknown-unknown`
//! `trunk build --release --public-url utca`
//!
//! [Determination of the Positional-Species Composition of Plant Reserve
//! Triacylglycerols by Partial Chemical
//! Deacylation](https://sci-hub.ru/10.1023/A:1016732708350)

#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(associated_type_defaults)]
#![feature(decl_macro)]
#![feature(float_next_up_down)]
#![feature(hash_extract_if)]
#![feature(impl_trait_in_assoc_type)]
#![feature(option_get_or_insert_default)]
#![feature(vec_into_raw_parts)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use app::App;

// When compiling natively
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> eframe::Result<()> {
    // std::env::set_var("POLARS_FMT_MAX_COLS", "256");
    // std::env::set_var("POLARS_FMT_MAX_ROWS", "32");
    // std::env::set_var("POLARS_FMT_TABLE_CELL_LIST_LEN", "256");
    // std::env::set_var("POLARS_FMT_STR_LEN", "256");

    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();
    eframe::run_native(
        "UTCA",
        Default::default(),
        Box::new(|context| Ok(Box::new(App::new(context)))),
    )
}

// When compiling to web using trunk
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();
    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                Default::default(),
                Box::new(|context| Ok(Box::new(App::new(context)))),
            )
            .await
            .expect("failed to start eframe");
    });
}

mod acylglycerol;
mod app;
mod r#const;
mod fatty_acid;
mod localization;
mod properties;
mod utils;
mod widgets;
