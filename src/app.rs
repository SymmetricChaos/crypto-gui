use crate::{cipher_id::CipherID, code_id::CodeID};
use eframe::{egui::{SidePanel, CentralPanel, ScrollArea, TopBottomPanel, Window, Button, Context, widgets}, epi};
use crate::cipher_panel::{ControlPanel, DisplayPanel};
use crate::text_types::PresetAlphabet::*;

// TODO: three way selector for About Page, Cipher Page, and Code Page
pub struct ClassicCrypto {
    control_panel: ControlPanel,
    display_panel: DisplayPanel,
    input: String,
    output: String,
    errors: String,
    active_cipher: CipherID,
    active_code: CodeID,
    show_alphabet_selector: bool,
    active_page: Page,
}

enum Page {
    About,
    Ciphers,
    Codes,
}

impl Default for ClassicCrypto {
    fn default() -> Self {
        Self { 
            control_panel: ControlPanel::default(),
            display_panel: DisplayPanel::default(),
            input: String::new(),
            output: String::new(),
            errors: String::new(),
            active_cipher: CipherID::default(),
            active_code: CodeID::default(),
            show_alphabet_selector: false,
            active_page: Page::Ciphers,
        }
    }
}

impl ClassicCrypto {
    fn cipher_page(&mut self, ctx: &Context) {
        SidePanel::right("cipher_display_panel").max_width(300.0).show(ctx, |ui| {
            self.display_panel.ui(ui, &mut self.input, &mut self.output, &mut self.errors, &mut self.active_cipher, &mut self.control_panel);
        
        });
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                self.control_panel.ui(ui, &mut self.active_cipher)
            });
        });
    }

    fn code_page(&mut self, ctx: &Context) {
        SidePanel::right("code_display_panel").max_width(300.0).show(ctx, |ui| {
            
        
        });
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                
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
                widgets::global_dark_light_mode_switch(ui);
                if ui.add(Button::new("Alphabets").small() ).clicked() {
                    self.show_alphabet_selector = !self.show_alphabet_selector;
                }
            });
        });

        // for (anchor, app) in self.apps.iter_mut() {
        //     if ui
        //         .selectable_label(self.selected_anchor == anchor, app.name())
        //         .clicked()
        //     {
        //         self.selected_anchor = anchor.to_owned();
        //         if frame.is_web() {
        //             ui.output().open_url(format!("#{}", anchor));
        //         }
        //     }
        // }

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
}
