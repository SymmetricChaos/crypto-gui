use std::{collections::BTreeSet};

use eframe::egui::{self, CtxRef};

use super::{caesar_panel::CaesarWindow, affine_panel::AffineWindow, substitution_panel::SubstitutionWindow, decorder_ring_panel::DecoderRingWindow};

/// Something to view in the cipher windows
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

/// Something to view
pub trait CipherFrame {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool);
}

pub enum CipherName {
    Caesar,
    Affine,
    General,
}

pub struct Ciphers {
    ciphers: Vec<Box<dyn CipherFrame>>,
    open: BTreeSet<String>,
}

impl Default for Ciphers {
    fn default() -> Self {
        Self::from_ciphers(vec![
            Box::new(CaesarWindow::default()),
            Box::new(AffineWindow::default()),
            Box::new(SubstitutionWindow::default()),
            Box::new(DecoderRingWindow::default())])
    }
}

impl Ciphers {
    pub fn from_ciphers(ciphers: Vec<Box<dyn CipherFrame>>) -> Self {
        let open = BTreeSet::new();
        Self { ciphers, open }
    }

    pub fn windows(&mut self, ctx: &egui::CtxRef) {
        let Self { ciphers, open } = self;
        for cipher in ciphers {
            let mut is_open = open.contains(cipher.name());
            cipher.show(ctx, &mut is_open);
            set_open(open, cipher.name(), is_open);
        }
    }

    pub fn checkboxes(&mut self, ui: &mut egui::Ui) {
        let Self { ciphers, open } = self;
        for cipher in ciphers {
            let mut is_open = open.contains(cipher.name());
            ui.checkbox(&mut is_open, cipher.name());
            set_open(open, cipher.name(), is_open);
        }
    }
}

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}



pub struct CipherWindows {
    ciphers: Ciphers,
}

impl Default for CipherWindows {
    fn default() -> Self {
        Self { ciphers: Ciphers::default() }
    }
}

impl CipherWindows {

    pub fn ui(&mut self, ctx: &CtxRef) {
        let Self { ciphers } = self;

        egui::SidePanel::left("selector_panel").show(ctx, |ui|{
            ciphers.checkboxes(ui);
        });

        self.windows(ctx)
        
    }

    fn windows(&mut self, ctx: &CtxRef) {
        let Self{ ciphers } = self;
        ciphers.windows(ctx)
    }
}
