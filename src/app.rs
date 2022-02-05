use crate::cipher_id::CipherID;
use eframe::{egui::{CtxRef, SidePanel, CentralPanel, ScrollArea, TopBottomPanel, Window, Button}, epi};

use crate::cipher_panel::{ControlPanel, DisplayPanel};


pub struct ClassicCrypto {
    control: ControlPanel,
    display: DisplayPanel,
    input: String,
    output: String,
    errors: String,
    active_cipher: CipherID,
    show_settings: bool,
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
        }
    }
}


impl epi::App for ClassicCrypto {
    fn name(&self) -> &str {
        "Classical Cryptography"
    }

    fn setup(
        &mut self,
        ctx: &CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let x = ctx.fonts().pixels_per_point();
    }

    fn update(&mut self, ctx: &CtxRef, frame: &epi::Frame) {
        frame.set_window_size((1000.0,550.0).into());
        ctx.set_pixels_per_point(1.2);
        
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if ui.add(Button::new("Settings").small() ).clicked() {
                self.show_settings = !self.show_settings;
            }
        });

        Window::new("🔧 Settings")
            .open(&mut self.show_settings)
            .vscroll(true)
            .show(ctx, |ui| {
                ctx.settings_ui(ui);
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