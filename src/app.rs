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
    show_about: bool,
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
            show_about: false,
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

    fn update(&mut self, ctx: &Context, _: &epi::Frame) {
        
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                if ui.add(Button::new("Settings").small() ).clicked() {
                    self.show_settings = !self.show_settings;
                }
                if ui.add(Button::new("About").small() ).clicked() {
                    self.show_about = !self.show_about;
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
            .open(&mut self.show_about)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label("Welcome to Classic Crypto an online cipher machine made using");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label("!");
                });
                ui.label("\n\nThis project starts 'classical cryptography' as early as writing itself and ends it in 1949 with the publication of 'Communication Theory of Secrecy Systems' by Claude Shannon at Bell Labs which introduced the modern theory of cryptography. The ciphers presented here are for historical interest not for use in security. Most can be broken by hand in less than a day and all of them can quickly be broken by computer.\n\n");
                ui.hyperlink_to("Check out the source code", "https://github.com/SymmetricChaos/crypto-gui");
                
        });

        SidePanel::right("display_panel").max_width(300.0).show(ctx, |ui| {
            self.display.ui(ui, &mut self.input, &mut self.output, &mut self.errors, &mut self.active_cipher, &mut self.control);
            
        });

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                self.control.ui(ui, &mut self.active_cipher)
            });
        });
    }
}