use crate::{
    cipher_panel::CipherInterface,
    code_panel::CodeInterface,
    hasher_panel::HasherInterface,
    pages::{io_panel::IOPanel, Page, TextPrepPage},
    rng_panel::RngInterface,
    ui_elements::UiElements,
};
use ciphers::ids::CipherId;
use codes::ids::CodeId;
use eframe::{
    egui::{
        self, warn_if_debug_build, widgets, CentralPanel, Context, FontData, FontDefinitions,
        RichText, ScrollArea, SidePanel, TopBottomPanel,
    },
    epaint::FontFamily,
    App,
};
use hashers::ids::HasherId;
use rngs::ids::RngId;

fn load_font(name: &str, family: &FontFamily, font_data: FontData, font_def: &mut FontDefinitions) {
    font_def.font_data.insert(name.into(), font_data);
    font_def.families.get_mut(family).unwrap().push(name.into());
}

pub struct ClassicCrypto {
    cipher_interface: CipherInterface,
    code_interface: CodeInterface,
    rng_interface: RngInterface,
    hasher_interface: HasherInterface,

    io_panel: IOPanel,
    input: String,
    output: String,
    errors: String,

    active_cipher: Option<CipherId>,
    active_code: Option<CodeId>,
    active_rng: Option<RngId>,
    active_hasher: Option<HasherId>,

    active_page: Page,
    text_prep_page: TextPrepPage,
}

impl Default for ClassicCrypto {
    fn default() -> Self {
        Self {
            // Input, output, and error shared by all Ciphers and Codes
            input: String::new(),
            output: String::new(),
            errors: String::new(),

            // IO Panel shared by all Ciphers and Codes
            io_panel: IOPanel::default(),

            // ID of the active Cipher or Code
            active_cipher: None,
            active_code: None,
            active_rng: None,
            active_hasher: None,

            // Which page we are on
            active_page: Page::About,

            // Contents of the TextPrepPage
            text_prep_page: TextPrepPage::default(),

            // Contains each Cipher and Code along with with controls and a panel for selecting them
            cipher_interface: CipherInterface::default(),
            code_interface: CodeInterface::default(),
            rng_interface: RngInterface::default(),
            hasher_interface: HasherInterface::default(),
        }
    }
}

impl ClassicCrypto {
    // Configure the CreationContext and also build the app
    pub fn build_with_context(cc: &eframe::CreationContext<'_>) -> Self {
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
            &FontFamily::Monospace,
            FontData::from_static(include_bytes!("../NotoSansJP-Regular.ttf")),
            &mut font_def,
        );
        load_font(
            "NotoJP",
            &FontFamily::Proportional,
            FontData::from_static(include_bytes!("../NotoSansJP-Regular.ttf")),
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
    // fn blank_page(&mut self, ctx: &Context) {
    //     CentralPanel::default().show(ctx, |ui| {
    //         ScrollArea::vertical().show(ui, |ui| {
    //             ui.label("<<<THIS PAGE INTENTIONALLY LEFT BLANK>>>")
    //         });
    //     });
    // }

    // Combox boxes for selecting codes
    fn code_selector_panel(&mut self, ctx: &Context) {
        SidePanel::left("code_selector_panel")
            .default_width(300.0)
            .min_width(100.0)
            .show(ctx, |ui| {
                ui.add_space(32.0);
                self.code_interface.combo_boxes(ui, &mut self.active_code)
            });
    }

    fn code_page(&mut self, ctx: &Context) {
        if self.active_page == Page::Code {
            self.code_selector_panel(ctx);
            SidePanel::right("code_io_panel")
                .default_width(300.0)
                .min_width(200.0)
                .show(ctx, |ui| {
                    self.io_panel.ui(
                        ui,
                        &mut self.input,
                        &mut self.output,
                        &mut self.errors,
                        &mut self.active_page,
                        &mut self.active_cipher,
                        &mut self.active_code,
                        &mut self.active_rng,
                        &mut self.active_hasher,
                        &mut self.cipher_interface,
                        &mut self.code_interface,
                        &mut self.rng_interface,
                        &mut self.hasher_interface,
                    );
                });

            CentralPanel::default().show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    match self.active_code {
                        Some(code) => {
                            ui.label(RichText::from(code.to_string()).heading());
                            ui.label(RichText::new(code.description()).size(12.0));
                            ui.add_space(16.0);
                            ui.separator();
                            ui.add_space(16.0);
                            self.code_interface
                                .get_active_code(&code)
                                .ui(ui);
                        }
                        None => {
                            ui.label(RichText::from("Codes").heading());
                            ui.label(RichText::new("A code is a method of transforming information in one form to another form.").size(12.0));
                            ui.add_space(16.0);
                            ui.separator();
                            ui.add_space(16.0);
                            ui.mono_strong("Codes exist for many reasons but the most common is ease of use. Often information in one form is difficult to use for a particular purpose and so needs to be changed. A spoken explanation vanishes in moments and travels only a short distance while the same explanation encoded as symbols can survive for years and be transported thousands of kilometers. Those same symbols become a problem for transmitting electronically so instead they may be enoded again as a sequence of signals, often just binary bits. The reciever of those bits then transforms then back to symbols and the reader interprets the symbols using their knowledge of how they relate to spoken language.");
                        }
                    };
                });
            });
        } else {
            // If somehow we are here without Page::Code selected
            self.code_selector_panel(ctx);
        }
    }

    // Combox boxes for selecting ciphers
    fn cipher_selector_panel(&mut self, ctx: &Context) {
        SidePanel::left("cipher_selector_panel")
            .default_width(300.0)
            .min_width(100.0)
            .show(ctx, |ui| {
                ui.add_space(32.0);
                self.cipher_interface
                    .combo_boxes(ui, &mut self.active_cipher)
            });
    }

    fn cipher_page(&mut self, ctx: &Context) {
        if self.active_page == Page::Cipher {
            self.cipher_selector_panel(ctx);

            SidePanel::right("cipher_io_panel")
                .default_width(300.0)
                .min_width(200.0)
                .show(ctx, |ui| {
                    self.io_panel.ui(
                        ui,
                        &mut self.input,
                        &mut self.output,
                        &mut self.errors,
                        &mut self.active_page,
                        &mut self.active_cipher,
                        &mut self.active_code,
                        &mut self.active_rng,
                        &mut self.active_hasher,
                        // &mut self.active_attack,
                        &mut self.cipher_interface,
                        &mut self.code_interface,
                        &mut self.rng_interface,
                        // &mut self.attack_interface,
                        &mut self.hasher_interface,
                    );
                });

            CentralPanel::default().show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    match self.active_cipher {
                        Some(cipher) => {
                            ui.label(RichText::from(cipher.to_string()).heading());
                            ui.label(RichText::new(cipher.description()).size(12.0));
                            ui.add_space(16.0);
                            ui.separator();
                            ui.add_space(16.0);
                            self.cipher_interface
                                .get_active_cipher(&cipher)
                                .ui(ui, &mut self.errors);
                        }
                        None => {
                            ui.label(RichText::from("Ciphers").heading());
                            ui.label(RichText::new("A cipher is a method of making information inaccessible to those without knowledge of a secret key.").size(12.0));
                            ui.add_space(16.0);
                            ui.separator();
                            // ui.add_space(16.0);
                            // ui.label(mono_strong("<<<INTERFACE>>>"));
                        }
                    };
                });
            });

        // If somehow we are here without Page::Cipher selected
        } else {
            self.cipher_selector_panel(ctx);
        }
    }

    // Combox boxes for selecting rng
    fn rng_selector_panel(&mut self, ctx: &Context) {
        SidePanel::left("rng_selector_panel")
            .default_width(300.0)
            .min_width(100.0)
            .show(ctx, |ui| {
                ui.add_space(32.0);
                self.rng_interface.combo_boxes(ui, &mut self.active_rng)
            });
    }

    fn rng_page(&mut self, ctx: &Context) {
        if self.active_page == Page::Rng {
            self.rng_selector_panel(ctx);

            SidePanel::right("rng_io_panel")
                .default_width(300.0)
                .min_width(200.0)
                .show(ctx, |ui| {
                    self.io_panel.ui(
                        ui,
                        &mut self.input,
                        &mut self.output,
                        &mut self.errors,
                        &mut self.active_page,
                        &mut self.active_cipher,
                        &mut self.active_code,
                        &mut self.active_rng,
                        &mut self.active_hasher,
                        // &mut self.active_attack,
                        &mut self.cipher_interface,
                        &mut self.code_interface,
                        &mut self.rng_interface,
                        // &mut self.attack_interface,
                        &mut self.hasher_interface,
                    );
                });

            CentralPanel::default().show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    match self.active_rng {
                        Some(rng) => {
                            ui.label(RichText::from(rng.to_string()).heading());
                            ui.label(RichText::new(rng.description()).size(12.0));
                            ui.add_space(16.0);
                            ui.separator();
                            ui.add_space(16.0);
                            self.rng_interface
                                .get_active_rng(&rng)
                                .ui(ui, &mut self.errors);
                        }
                        None => {
                            ui.label(RichText::from("Random Number Generators").heading());
                            ui.label(RichText::new("Random number generators are methods of creating seemingly random numbers.").size(12.0));
                            ui.add_space(16.0);
                            ui.separator();
                            ui.add_space(16.0);
                            ui.label(RichText::new("Truly random numbers can be defined mathematically but it is unclear if such things exist in reality. When something similar to truly random numbers is needed hardware RNGs are used which extract randomness from some physical process such as electrical noise or radioactive decacy. However hardware random number generation lacks the potentially valuable properties of speed, portability, and repeatability so for many purposes pseudorandom numbers are preferred. These are numbers which pass statistical tests of randomness but are created by an algorithm. The fastest modern PRNGs (such as the xoshiro family) can produce several gigabytes per second and run on even very simple processors while passing stringent tests. Algorithms are also used to produce quasirandom numbers which pass few statistical tests but possess some important properties in common with random numbers. The most desirable property of quasirandom numbers is low-discrepancy, meaning that they cover an area more evenly than random or uniformly spaced points do in the sense that it is more difficult to draw a box that contains an unusually large or small number of points for its volume.").size(12.0));
                        }
                    };
                });
            });

        // If somehow we are here without Page::Rng selected
        } else {
            self.rng_selector_panel(ctx);
        }
    }

    // Combox boxes for selecting hash function
    fn hash_selector_panel(&mut self, ctx: &Context) {
        SidePanel::left("hash_selector_panel")
            .default_width(300.0)
            .min_width(100.0)
            .show(ctx, |ui| {
                ui.add_space(32.0);
                self.hasher_interface
                    .combo_boxes(ui, &mut self.active_hasher)
            });
    }
    fn hash_page(&mut self, ctx: &Context) {
        if self.active_page == Page::Hash {
            self.hash_selector_panel(ctx);

            SidePanel::right("hash_io_panel")
                .default_width(300.0)
                .min_width(200.0)
                .show(ctx, |ui| {
                    self.io_panel.ui(
                        ui,
                        &mut self.input,
                        &mut self.output,
                        &mut self.errors,
                        &mut self.active_page,
                        &mut self.active_cipher,
                        &mut self.active_code,
                        &mut self.active_rng,
                        &mut self.active_hasher,
                        // &mut self.active_attack,
                        &mut self.cipher_interface,
                        &mut self.code_interface,
                        &mut self.rng_interface,
                        // &mut self.attack_interface,
                        &mut self.hasher_interface,
                    );
                });

            CentralPanel::default().show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    match self.active_hasher {
                        Some(hasher) => {
                            ui.label(RichText::from(hasher.to_string()).heading());
                            ui.label(RichText::new(hasher.description()).size(12.0));
                            ui.add_space(16.0);
                            ui.separator();
                            ui.add_space(16.0);
                            self.hasher_interface
                                .get_active_hasher(&hasher)
                                .ui(ui, &mut self.errors);
                        }
                        None => {
                            ui.label(RichText::from("Hash Functions").heading());
                            ui.label(RichText::new("Hash functions take an arbitrary amount of data and map it onto a value of a fixed size.").size(12.0));
                            ui.add_space(16.0);
                            ui.separator();
                            ui.add_space(16.0);
                            ui.label(RichText::new("Often it is desireable for the mapping of a hash function to be highly chaotic, so that even similar inputs result in different output, and hundreds of bits in length, to make it unlikely that two inputs will have the same hash. In some cases, however, a hash function can be as simple as taking the low bits of a number.").size(12.0));
                        }
                    };
                });
            });

        // If somehow we are here without Page::Rng selected
        } else {
            self.hash_selector_panel(ctx);
        }
    }

    fn about_page(&mut self, ctx: &Context) {
        SidePanel::left("about_display_panel")
            .default_width(500.0)
            .min_width(200.0)
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
            ui.label( RichText::new("Classical Cryptography").heading().strong());
            ui.label("The era of classical cryptography dates back to at least the invention of written language. In societies with low literacy writing itself was often a secure form of encryption as it was incomprehensible to most people. The era ends in the middle of the 20th century with Claude Shannon's publication of the paper 'Communication Theory of Secrecy Systems' at Bell Labs which established the modern theory of cryptography and contained and early mathematics proof of the security of an encryption system, the one-time pad. The pre-modern ciphers presented here were mostly based on an intuitive sense of what would be difficult for the enemy to decipher and limitations of what the person encrypting the message could accomplish by hand or with a simple tool. This changed slightly in the early 20th century when improvements in engineering caused the rise of electromechanical devices, such as the famous Enigma Mahchine, that could rapidly perform encryption not feasible to do by hand.");
            ui.add_space(16.0);
            ui.label(RichText::from("A Note on the Terms Cipher and Code").size(16.0).strong());
            ui.label("No strong distinction is made in literature between a 'cipher' and a 'code' in this era. However this project adopts the modern convention that a cipher has a changeable key and a code does not. That is: to understand a cipher one must know both the method as some secret additional information while a code can be read by anyone who knows the method of encoding.");
            ui.add_space(16.0);
            ui.label(RichText::from("Supported Text Characters").size(16.0).strong());
            ui.label("Most Latin alphabets are supported by the site.\n\nIt is important to note that all ciphers operated on Rust's character type which represents a single Unicode codepoint. Not all graphemes are formed from single code points. Text can be normalized on the Text Prep page.");
        });
    }
}

impl App for ClassicCrypto {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                widgets::global_dark_light_mode_switch(ui);
                ui.separator();

                if ui.button("About").clicked() {
                    self.active_page = Page::About;
                }
                if ui.button("Ciphers").clicked() {
                    self.active_page = Page::Cipher;
                    self.active_cipher = None;
                }
                if ui.button("Codes").clicked() {
                    self.active_page = Page::Code;
                    self.active_code = None;
                }
                if ui.button("RNGs").clicked() {
                    self.active_page = Page::Rng;
                    self.active_rng = None;
                }
                if ui.button("Hashers").clicked() {
                    self.active_page = Page::Hash;
                    self.active_hasher = None;
                }
                if ui.button("Text").clicked() {
                    self.active_page = Page::TextPrep;
                }
            });
        });

        match self.active_page {
            Page::About => self.about_page(ctx),
            Page::Cipher => self.cipher_page(ctx),
            Page::Code => self.code_page(ctx),
            Page::TextPrep => self.text_prep_page(ctx),
            Page::Rng => self.rng_page(ctx),
            Page::Hash => self.hash_page(ctx),
            //_ => self.blank_page(ctx),
        }
    }
}
