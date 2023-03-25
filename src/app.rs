use crate::cipher_panel::CipherInterface;
use crate::code_panel::CodeInterface;
use crate::ids::{CipherID, CodeID};
use crate::pages::io_panel::IOPanel;
use crate::pages::{Page, TextPrepPage};
use eframe::egui;
use eframe::{
    egui::{
        warn_if_debug_build, widgets, CentralPanel, Context, FontData, FontDefinitions, RichText,
        ScrollArea, SelectableLabel, SidePanel, TopBottomPanel, Ui,
    },
    epaint::FontFamily,
    App,
};

fn page_selector(ui: &mut Ui, name: &str, selected_page: Page, active_page: &mut Page) {
    if ui
        .add(SelectableLabel::new(active_page == &selected_page, name))
        .clicked()
    {
        *active_page = selected_page
    }
}

fn load_font(name: &str, family: &FontFamily, font_data: FontData, font_def: &mut FontDefinitions) {
    font_def.font_data.insert(name.into(), font_data);
    font_def.families.get_mut(family).unwrap().push(name.into());
}

pub struct ClassicCrypto {
    cipher_interface: CipherInterface,
    code_interface: CodeInterface,
    // cipher_display_panel: CipherIO,
    // code_control_panel: CodeControlPanel,
    // code_display_panel: CodeDisplayPanel,
    // rng_display_panel: RngInfoPage,
    io_panel: IOPanel,
    input: String,
    output: String,
    errors: String,

    active_cipher: CipherID,
    active_code: CodeID,
    // active_rng: RngID,
    active_page: Page,
    text_prep_page: TextPrepPage,
}

impl Default for ClassicCrypto {
    fn default() -> Self {
        Self {
            // Input, output, and error shared by Ciphers and Codes
            input: String::new(),
            output: String::new(),
            errors: String::new(),

            // IO Panel shared by Ciphers and Codes
            io_panel: IOPanel::default(),

            // Which cipher is active
            active_cipher: CipherID::default(),
            active_code: CodeID::default(),

            // Which page we are on
            active_page: Page::About,

            text_prep_page: TextPrepPage::default(),

            // Interface that hold a copy of each cipher and organizes them
            cipher_interface: CipherInterface::default(),
            code_interface: CodeInterface::default(),
        }
    }
}

impl ClassicCrypto {
    // Configure the CreationContext and also build the app
    pub fn build_with_context(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let mut font_def = FontDefinitions::default();

        // Noto fonts to get wide coverage, more can be added if needed
        load_font(
            "NotoMono",
            &FontFamily::Monospace,
            FontData::from_static(include_bytes!("../NotoSansMono-Regular.ttf")),
            &mut font_def,
        );
        load_font(
            "NotoSans",
            &FontFamily::Proportional,
            FontData::from_static(include_bytes!("../NotoSans-Regular.ttf")),
            &mut font_def,
        );
        load_font(
            "NotoSymbols",
            &FontFamily::Proportional,
            FontData::from_static(include_bytes!("../NotoSansSymbols-Regular.ttf")),
            &mut font_def,
        );
        load_font(
            "NotoSymbols2",
            &FontFamily::Proportional,
            FontData::from_static(include_bytes!("../NotoSansSymbols2-Regular.ttf")),
            &mut font_def,
        );
        load_font(
            "NotoMath",
            &FontFamily::Proportional,
            FontData::from_static(include_bytes!("../NotoSansMath-Regular.ttf")),
            &mut font_def,
        );
        load_font(
            "NotoJP",
            &FontFamily::Proportional,
            FontData::from_static(include_bytes!("../NotoSansJP-Regular.otf")),
            &mut font_def,
        );
        load_font(
            "Segoe",
            &FontFamily::Monospace,
            FontData::from_static(include_bytes!("../seguisym.ttf")),
            &mut font_def,
        );

        cc.egui_ctx.set_fonts(font_def);

        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self::default()
    }

    fn text_prep_page(&mut self, ctx: &Context) {
        self.text_prep_page.view(&ctx)
    }

    // Direct invalid selections here
    fn blank_page(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| ui.label("you have reached this page in error"));
        });
    }

    // Combox boxes for selecting codes
    fn code_selector_panel(&mut self, ctx: &Context) {
        SidePanel::left("code_selector_panel")
            .default_width(150.0)
            .min_width(100.0)
            .max_width(200.0)
            .show(ctx, |ui| {
                self.code_interface.combo_boxes(ui, &mut self.active_code)
            });
    }

    fn code_page(&mut self, ctx: &Context) {
        if self.active_page == Page::Code {
            self.code_selector_panel(ctx);

            SidePanel::right("code_io_panel")
                .default_width(150.0)
                .min_width(100.0)
                .max_width(200.0)
                .show(ctx, |ui| {
                    self.io_panel.ui(
                        ui,
                        &mut self.input,
                        &mut self.output,
                        &mut self.errors,
                        &mut self.active_page,
                        &mut self.active_cipher,
                        &mut self.active_code,
                        &mut self.cipher_interface,
                        &mut self.code_interface,
                    );
                });

            CentralPanel::default().show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    let name = RichText::new(String::from(self.active_code))
                        .strong()
                        .heading();
                    ui.add(egui::Label::new(name));
                    ui.label(self.active_code.description());

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(16.0);
                    self.code_interface
                        .get_active_code(&self.active_code)
                        .ui(ui, &mut self.errors)
                });
            });

        // If somehow we are here without Page::Code selected
        } else {
            self.blank_page(ctx)
        }
    }

    // Combox boxes for selecting ciphers
    fn cipher_selector_panel(&mut self, ctx: &Context) {
        SidePanel::left("cipher_selector_panel")
            .default_width(150.0)
            .min_width(100.0)
            .max_width(200.0)
            .show(ctx, |ui| {
                self.cipher_interface
                    .combo_boxes(ui, &mut self.active_cipher)
            });
    }

    fn cipher_page(&mut self, ctx: &Context) {
        if self.active_page == Page::Cipher {
            self.cipher_selector_panel(ctx);

            SidePanel::right("cipher_io_panel")
                .default_width(150.0)
                .min_width(100.0)
                .max_width(200.0)
                .show(ctx, |ui| {
                    self.io_panel.ui(
                        ui,
                        &mut self.input,
                        &mut self.output,
                        &mut self.errors,
                        &mut self.active_page,
                        &mut self.active_cipher,
                        &mut self.active_code,
                        &mut self.cipher_interface,
                        &mut self.code_interface,
                    );
                });

            CentralPanel::default().show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    let name = RichText::new(String::from(self.active_cipher))
                        .strong()
                        .heading();
                    ui.add(egui::Label::new(name));
                    ui.label(self.active_cipher.description());

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(16.0);
                    self.cipher_interface
                        .get_active_cipher(&self.active_cipher)
                        .ui(ui, &mut self.errors)
                });
            });

        // If somehow we are here without Page::Cipher selected
        } else {
            self.blank_page(ctx)
        }
    }

    fn about_page(&mut self, ctx: &Context) {
        SidePanel::left("about_display_panel")
            .default_width(500.0)
            .max_width(500.0)
            .show(ctx, |ui| {
                warn_if_debug_build(ui);
                let hello =
                    RichText::new("Welcome to Classic Crypto!\nCheck out the Ciphers available.")
                        .strong();
                ui.label(hello);
                ui.add_space(20.0);
                ui.hyperlink_to(
                    "source code",
                    "https://github.com/SymmetricChaos/crypto-gui",
                );
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
            ui.add_space(16.0);
            let grapheme_subhead = RichText::new("Supported Text Characters").strong();
            ui.label(grapheme_subhead);
            ui.label("Most Latin alphabets are supported by the site.\n\nIt is important to note that all ciphers operated on Rust's character type which represents a single Unicode codepoint. Not all graphemes are formed from single code points. Text can be normalized on the Text Prep page.");
        });
    }
}

impl App for ClassicCrypto {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        //frame.set_window_size((900.0, 700.0).into());

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                widgets::global_dark_light_mode_switch(ui);
                ui.separator();

                page_selector(ui, "About", Page::About, &mut self.active_page);
                page_selector(ui, "Ciphers", Page::Cipher, &mut self.active_page);
                page_selector(ui, "Codes", Page::Code, &mut self.active_page);
                // page_selector(ui, "RNGs", Page::Rng(None), &mut self.active_page);
                page_selector(ui, "Text", Page::TextPrep, &mut self.active_page);
            });
        });

        match self.active_page {
            Page::About => self.about_page(ctx),
            Page::Cipher => self.cipher_page(ctx),
            Page::Code => self.code_page(ctx),
            // Page::Rng(_) => todo!("make a method for the RNG page"),
            // Page::CipherCategory => self.cipher_category_page(ctx),
            Page::TextPrep => self.text_prep_page(ctx),
            _ => self.blank_page(ctx),
        }
    }
}
