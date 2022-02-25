use crate::cipher_id::CipherID;
use eframe::{egui::{SidePanel, CentralPanel, ScrollArea, TopBottomPanel, Window, Button, Context}, epi};
use crate::cipher_panel::{ControlPanel, DisplayPanel};


pub struct ClassicCrypto {
    control: ControlPanel,
    display: DisplayPanel,
    input: String,
    output: String,
    errors: String,
    active_cipher: CipherID,
    show_settings: bool,
    about: bool,
}

impl Default for ClassicCrypto {
    fn default() -> Self {
        Self { 
            control: ControlPanel::default(),
            display: DisplayPanel::default(),
            input: String::new(),
            output: String::new(),
            errors: String::new(),
            active_cipher: CipherID::default(),
            show_settings: false,
            about: true,
        }
    }
}


impl epi::App for ClassicCrypto {
    fn name(&self) -> &str {
        "Classical Cryptography"
    }

    fn setup(
        &mut self,
        _ctx: &Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    fn update(&mut self, ctx: &Context, frame: &epi::Frame) {
        frame.set_window_size((1000.0,550.0).into());
        ctx.set_pixels_per_point(1.2);
        
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                if ui.add(Button::new("Settings").small() ).clicked() {
                    self.show_settings = !self.show_settings;
                }
                if ui.add(Button::new("About").small() ).clicked() {
                    self.about = !self.about;
                }
            });
        });

        Window::new("🔧 Settings")
            .open(&mut self.show_settings)
            .vscroll(true)
            .show(ctx, |ui| {
                ctx.settings_ui(ui);
        });

        Window::new("About")
            .open(&mut self.about)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.label("Welcome to Classic Crypto an online cipher machine made using egui, Rust, and WASM!\n\nThis project starts 'classical cryptography' as early as writing itself and ends it in 1949 with the publication of 'Communication Theory of Secrecy Systems' by Claude Shannon at Bell Labs which introduct the modern theory of cryptography. Most of the ciphers here can be broken by hand in less than a day and all of them can be broken by computer in a few moments.");
        });

        SidePanel::right("display_panel").max_width(300.0).show(ctx, |ui| {
            self.display.ui(ui, &mut self.input, &mut self.output, &mut self.errors);
            
        });

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                self.control.ui(ui, &mut self.input, &mut self.output, &mut self.errors, &mut self.active_cipher)
            });
        });
    }
}