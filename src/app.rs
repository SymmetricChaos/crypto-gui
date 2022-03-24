use crate::{cipher_id::CipherID, code_id::CodeID};
use eframe::{egui::{SidePanel, CentralPanel, ScrollArea, TopBottomPanel, Context, widgets, SelectableLabel, warn_if_debug_build, RichText, FontDefinitions, FontData}, epi, epaint::FontFamily};
//use crate::text_types::PresetAlphabet::*;
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
    //show_alphabet_selector: bool,
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
            //show_alphabet_selector: false,
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
        SidePanel::left("about_display_panel").max_width(500.0).show(ctx, |ui| {
            warn_if_debug_build(ui);
            let hello = RichText::new("Welcome to Classic Crypto!\nCheck out the Ciphers and Codes available.").strong();
            ui.label(hello);
            ui.add_space(20.0);
            ui.hyperlink_to("source code", "https://github.com/SymmetricChaos/crypto-gui");
            ui.add_space(10.0);
            ui.hyperlink_to("powered by egui", "https://github.com/emilk/egui");
        });
        CentralPanel::default().show(ctx, |ui| {
            let title = RichText::new("Classical Cryptography").heading();
            ui.label(title);
            ui.label("The era of classical cryptography dates back to at least the invention of written language. In societies with low literacy writing itself was often a secure form of encryption as it was incomprehensible to most people. The era ends in the middle of the 20th century with Claude Shannon's publication of the paper 'Communication Theory of Secrecy Systems' at Bell Labs which established the modern theory of cryptography and contained and early mathematics proof of the security of an encryption system, the one-time pad. The pre-modern ciphers presented here were mostly based on an intuitive sense of what would be difficult for the enemy to decipher and limitations of what the person encrypting the message could accomplish by hand or with a simple tool. This changed slightly in the early 20th century when improvements in engineering caused the rise of electromechanical devices, such as the famous Enigma Mahchine, that could rapidly perform encryption not feasible to do by hand.");
            ui.add_space(16.0);
            let cipher_code_subhead = RichText::new("A Note on the Terms Cipher and Code").strong();
            ui.label(cipher_code_subhead);
            ui.label("No strong distinction is made in literature between a 'cipher' and a 'code' in this era. However this project adopts the modern convention that a cipher has a changeable key and a code does not. That is: to understand a cipher one must know both the method as some secret additional information while a code can be read by anyone who knows the method of encoding.");
            ui.label(format!("{:?}",ctx.fonts().families()));
        });
    }

    fn configure_font(&self, ctx: &Context) {
        let mut font_def = FontDefinitions::default();
        // Load FreeMono.ttf and use it at the main monospace font
        font_def.font_data.insert(
            "FreeMonoTTF".into(), 
            FontData::from_static(include_bytes!("..\\FreeMono.ttf"))
        );
        font_def
            .families
            .get_mut(&FontFamily::Monospace).unwrap()
            .insert(0, "FreeMonoTTF".into()
        );

        // Fallback on FreeMono.otf
        font_def.font_data.insert(
            "FreeMonoOTF".into(), 
            FontData::from_static(include_bytes!("..\\FreeMono.otf"))
        );
        font_def
            .families
            .get_mut(&FontFamily::Monospace).unwrap()
            .push("FreeMonoOTF".into()
        );
        
        ctx.set_fonts(font_def);
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

                // ui.separator();
                // if ui.add(Button::new("Alphabets").small() ).clicked() {
                //     self.show_alphabet_selector = !self.show_alphabet_selector;
                // }
            });
        });

        // Window::new("Settings")
        //     .open(&mut self.show_settings)
        //     .vscroll(true)
        //     .show(ctx, |ui| {
        //         ctx.settings_ui(ui);
        // });

        // Window::new("Alphabet Selector")
        //     .open(&mut self.show_alphabet_selector)
        //     .vscroll(true)
        //     .show(ctx, |ui| {
        //         ui.label("Click to Copy");
        //         ui.add_space(16.0);
        //         for alphabet in [BasicLatin, BasicLatinNoJ, BasicLatinNoQ, BasicLatinWithDigits] {
        //             if ui.button(String::from(alphabet)).clicked() {
        //                 ui.output().copied_text = String::from(alphabet);
        //             };
        //         }
        // });

        match self.active_page {
            Page::About => self.about_page(ctx),
            Page::Ciphers => self.cipher_page(ctx),
            Page::Codes => self.code_page(ctx),
        }
    }

    fn setup(
        &mut self,
        ctx: &Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        self.configure_font(ctx);
    }

    fn name(&self) -> &str {
        "Classical Cryptography"
    }
}
