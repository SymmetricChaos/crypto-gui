pub mod app;

pub mod ids;

pub mod cipher_panel;
pub mod code_panel;

// pub mod cipher_attacks;
// pub mod attack_panel;

pub mod pages;

pub mod ui_elements;

// pub mod global_rng;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Classic Cryptography",
        native_options,
        Box::new(|cc| Box::new(app::ClassicCrypto::build_with_context(cc))),
    )
    .expect("failed to start eframe");
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(app::ClassicCrypto::build_with_context(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
