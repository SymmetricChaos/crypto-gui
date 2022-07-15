use crate::cipher_panel::{CipherControlPanel, CipherDisplayPanel};
use crate::code_panel::{CodeControlPanel, CodeDisplayPanel};
use crate::pages::category_page::CipherCategoryPage;
use crate::pages::{CipherCategory, Page, TextPrepPage};
use crate::{cipher_id::CipherID, code_id::CodeID};
use eframe::egui;
use eframe::{
    egui::{
        warn_if_debug_build, widgets, CentralPanel, Context, FontData, FontDefinitions, RichText,
        ScrollArea, SelectableLabel, SidePanel, TopBottomPanel, Ui,
    },
    epaint::FontFamily,
    App,
};

fn page_selector(ui: &mut Ui, name: &str, page: Page, active_page: &mut Page) {
    if ui
        .add(SelectableLabel::new(active_page == &page, name))
        .clicked()
    {
        *active_page = page
    }
}

fn load_font(name: &str, family: &FontFamily, font_data: FontData, font_def: &mut FontDefinitions) {
    font_def.font_data.insert(
        name.into(),
        font_data
    );
    font_def
        .families
        .get_mut(family)
        .unwrap()
        .push(name.into());
}

pub struct ClassicCrypto {
    cipher_category: CipherCategory,
    cipher_control_panel: CipherControlPanel,
    cipher_display_panel: CipherDisplayPanel,
    code_control_panel: CodeControlPanel,
    code_display_panel: CodeDisplayPanel,
    input: String,
    output: String,
    errors: String,
    active_cipher: CipherID,
    active_code: CodeID,
    active_page: Page,
    text_prep_page: TextPrepPage,
    cipher_category_page: CipherCategoryPage,
}

impl Default for ClassicCrypto {
    fn default() -> Self {
        Self {
            cipher_control_panel: CipherControlPanel::default(),
            cipher_display_panel: CipherDisplayPanel::default(),
            code_control_panel: CodeControlPanel::default(),
            code_display_panel: CodeDisplayPanel::default(),
            input: String::new(),
            output: String::new(),
            errors: String::new(),
            active_cipher: CipherID::default(),
            active_code: CodeID::default(),
            active_page: Page::About,
            cipher_category: CipherCategory::Substituion,
            text_prep_page: TextPrepPage::default(),
            cipher_category_page: CipherCategoryPage::default(),
        }
    }
}

impl ClassicCrypto {
    // Configure the CreationContext and also build the app
    pub fn build_with_context(cc: &eframe::CreationContext<'_>) -> Self {
        let mut font_def = FontDefinitions::default();

        // Noto fonts to get wide coverage, more can be added if needed
        load_font("NotoMono", &FontFamily::Monospace, FontData::from_static(include_bytes!("../NotoSansMono-Regular.ttf")), &mut font_def);
        load_font("NotoSans", &FontFamily::Proportional, FontData::from_static(include_bytes!("../NotoSans-Regular.ttf")), &mut font_def);
        load_font("NotoSymbols", &FontFamily::Proportional, FontData::from_static(include_bytes!("../NotoSansSymbols-Regular.ttf")), &mut font_def);
        load_font("NotoSymbols2", &FontFamily::Proportional, FontData::from_static(include_bytes!("../NotoSansSymbols2-Regular.ttf")), &mut font_def);
        load_font("NotoMath", &FontFamily::Proportional, FontData::from_static(include_bytes!("../NotoSansMath-Regular.ttf")), &mut font_def);
        load_font("NotoJP", &FontFamily::Proportional, FontData::from_static(include_bytes!("../NotoSansJP-Regular.otf")), &mut font_def);

        cc.egui_ctx.set_fonts(font_def);

        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self::default()
    }

    fn text_prep_page(&mut self, ctx: &Context) {
        self.text_prep_page.view(&ctx)
    }

    fn cipher_category_page(&mut self, ctx: &Context) {
        self.cipher_category_page.view(
            &ctx,
            &mut self.cipher_category,
            &mut self.active_cipher,
            &mut self.active_page,
        )
    }

    fn cipher_page(&mut self, ctx: &Context) {
        SidePanel::right("cipher_display_panel")
            .max_width(300.0)
            .show(ctx, |ui| {
                self.cipher_display_panel.ui(
                    ui,
                    &mut self.input,
                    &mut self.output,
                    &mut self.errors,
                    &mut self.active_cipher,
                    &mut self.cipher_control_panel,
                );
            });
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                self.cipher_control_panel
                    .ui(ui, &mut self.active_cipher, &mut self.errors)
            });
        });
    }

    fn code_page(&mut self, ctx: &Context) {
        SidePanel::right("code_display_panel")
            .max_width(300.0)
            .show(ctx, |ui| {
                self.code_display_panel.ui(
                    ui,
                    &mut self.input,
                    &mut self.output,
                    &mut self.errors,
                    &mut self.active_code,
                    &mut self.code_control_panel,
                );
            });
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                self.code_control_panel.ui(
                    ui,
                    &mut self.active_code,
                    &mut self.input,
                    &mut self.output,
                    &mut self.errors,
                )
            });
        });
    }

    fn about_page(&mut self, ctx: &Context) {
        SidePanel::left("about_display_panel")
            .max_width(500.0)
            .show(ctx, |ui| {
                warn_if_debug_build(ui);
                let hello = RichText::new(
                    "Welcome to Classic Crypto!\nCheck out the Ciphers and Codes available.",
                )
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
            ui.label("While essentially all of Unicode can be displayed ciphers and codes operate on individual codepoints, unexpected behavior will occur when combining characters are used. Optional support for Unicode graphemes may be added later.");
        });
    }
}

impl App for ClassicCrypto {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                widgets::global_dark_light_mode_switch(ui);
                ui.separator();

                page_selector(ui, "About", Page::About, &mut self.active_page);
                page_selector(ui, "Ciphers", Page::CipherCategory, &mut self.active_page);
                page_selector(ui, "Codes", Page::Code, &mut self.active_page);
                page_selector(ui, "Text", Page::TextPrep, &mut self.active_page);
            });
        });

        match self.active_page {
            Page::About => self.about_page(ctx),
            Page::Cipher => self.cipher_page(ctx),
            Page::Code => self.code_page(ctx),
            Page::CipherCategory => self.cipher_category_page(ctx),
            Page::TextPrep => self.text_prep_page(ctx),
        }
    }
}
