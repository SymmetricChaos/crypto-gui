use crate::{cipher_id::CipherID, code_id::CodeID, code_panel};
use eframe::{egui::{SidePanel, CentralPanel, ScrollArea, TopBottomPanel, Window, Button, Context, widgets, SelectableLabel}, epi};
use crate::text_types::PresetAlphabet::*;
use crate::code_panel::{CodeDisplayPanel,CodeControlPanel};
use crate::cipher_panel::{CipherDisplayPanel,CipherControlPanel};

#[derive(Debug, PartialEq, Eq)]
enum Page {
    About,
    Ciphers,
    Codes,
}

pub struct ClassicCrypto {
    cipher_control_panel: CipherControlPanel,
    code_control_panel: CodeControlPanel,
    cipher_display_panel: CipherDisplayPanel,
    code_display_panel: CodeDisplayPanel,
    input: String,
    output: String,
    errors: String,
    active_cipher: CipherID,
    active_code: CodeID,
    show_alphabet_selector: bool,
    active_page: Page,
}


impl Default for ClassicCrypto {
    fn default() -> Self {
        Self { 
            cipher_control_panel: CipherControlPanel::default(),
            code_control_panel: CodeControlPanel::default(),
            cipher_display_panel: CipherDisplayPanel::default(),
            code_display_panel: CodeDisplayPanel::default(),
            input: String::new(),
            output: String::new(),
            errors: String::new(),
            active_cipher: CipherID::default(),
            active_code: CodeID::default(),
            show_alphabet_selector: false,
            active_page: Page::About,
        }
    }
}

impl ClassicCrypto {
    fn cipher_page(&mut self, ctx: &Context) {
        SidePanel::right("cipher_display_panel").max_width(300.0).show(ctx, |ui| {
            self.cipher_display_panel.ui(ui, &mut self.input, &mut self.output, &mut self.errors, &mut self.active_cipher, &mut self.cipher_control_panel);
        
        });
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                self.cipher_control_panel.ui(ui, &mut self.active_cipher)
            });
        });
    }

    fn code_page(&mut self, ctx: &Context) {
        SidePanel::right("code_display_panel").max_width(300.0).show(ctx, |ui| {
            self.code_display_panel.ui(ui, &mut self.input, &mut self.output, &mut self.errors, &mut self.active_code, &mut self.code_control_panel);
        
        });
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                self.code_control_panel.ui(ui, &mut self.active_code)
            });
        });
    }

    fn about_page(&mut self, ctx: &Context) {
        SidePanel::right("about_display_panel").max_width(300.0).show(ctx, |ui| {
            
        
        });
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                
            });
        });
    }
}


impl epi::App for ClassicCrypto {
    fn update(&mut self, ctx: &Context, _: &epi::Frame) {
        
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                widgets::global_dark_light_mode_switch(ui);
                ui.separator();
                if ui.add(SelectableLabel::new(self.active_page == Page::Ciphers, "Ciphers")).clicked() {
                    self.active_page = Page::Ciphers
                }
                if ui.add(SelectableLabel::new(self.active_page == Page::Codes, "Codes")).clicked() {
                    self.active_page = Page::Codes
                }
                if ui.add(SelectableLabel::new(self.active_page == Page::About, "About")).clicked() {
                    self.active_page = Page::About
                }
                // No idea why this is giving me an error
                // ui.selectable_label(&mut self.active_page, Page::Ciphers, "Ciphers");
                // ui.selectable_label(&mut self.active_page, Page::Codes, "Codes");
                // ui.selectable_label(&mut self.active_page, Page::About, "About");
                ui.separator();
                if ui.add(Button::new("Alphabets").small() ).clicked() {
                    self.show_alphabet_selector = !self.show_alphabet_selector;
                }
            });
        });

        // Window::new("Settings")
        //     .open(&mut self.show_settings)
        //     .vscroll(true)
        //     .show(ctx, |ui| {
        //         ctx.settings_ui(ui);
        // });

        Window::new("Alphabet Selector")
            .open(&mut self.show_alphabet_selector)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.label("Click to Copy");
                ui.add_space(16.0);
                for alphabet in [BasicLatin, BasicLatinNoJ, BasicLatinNoQ, BasicLatinWithDigits] {
                    if ui.button(String::from(alphabet)).clicked() {
                        ui.output().copied_text = String::from(alphabet);
                    };
                }

        });

        match self.active_page {
            Page::About => self.about_page(ctx),
            Page::Ciphers => self.cipher_page(ctx),
            Page::Codes => self.code_page(ctx),
        }
        
    }

    fn setup(
        &mut self,
        _ctx: &Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    fn name(&self) -> &str {
        "Classical Cryptography"
    }
}
