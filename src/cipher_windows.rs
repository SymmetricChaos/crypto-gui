/// Something to view in the cipher windows
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

/// Something to view
pub trait Cipher {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}


pub struct Ciphers {
    ciphers: Vec<Box<dyn Cipher>>,
    open: BTreeSet<String>,
}

impl Default for Ciphers {

}

impl Ciphers {
    pub fn from_ciphers(ciphers: Vec<Box<dyn Cipher>>) -> Self {
        let mut open = BTreeSet::new();
        open.insert(
            super::widget_gallery::WidgetGallery::default()
                .name()
                .to_owned(),
        );

        Self { ciphers, open }
    }

    pub fn windows(&mut self, ctx: &Context) {
        let Self { ciphers, open } = self;
        for cipher in ciphers {
            let mut is_open = open.contains(demo.name());
            cipher.show(ctx, &mut is_open);
            set_open(open, cipher.name(), is_open);
        }
    }
}



pub struct CipherWindows {
    ciphers: Ciphers,
}

impl CipherWindows {

    pub fn ui(&mut self, ctx: &Context) {
        let Self { ciphers } = self;

    }

}
