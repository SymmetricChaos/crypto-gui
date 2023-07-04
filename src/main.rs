pub mod app;

pub mod ids;

pub mod cipher_panel;
pub mod code_panel;

pub mod pages;

pub mod ui_elements;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Classic Cryptography",
        native_options,
        Box::new(|cc| Box::new(app::ClassicCrypto::build_with_context(cc))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "Classic Cryptography", // hardcode it
                web_options,
                Box::new(|cc| Box::new(app::ClassicCrypto::build_with_context(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
