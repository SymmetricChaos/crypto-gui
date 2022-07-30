use crate::rngs::FibLsfr;
use eframe::egui::Ui;

use super::View;

#[derive(Default)]
enum Selector {
    #[default]
    Eight,
    Sixteen,
    ThirtyTwo,
}

struct FibLsfrFrame {
    eight: FibLsfr<8>,
    sixteen: FibLsfr<16>,
    thirty_two: FibLsfr<32>,
    selector: Selector,
}

impl Default for FibLsfrFrame {
    fn default() -> Self {
        Self {
            eight: Default::default(),
            sixteen: Default::default(),
            thirty_two: Default::default(),
            selector: Default::default(),
        }
    }
}

impl View for FibLsfrFrame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        match self.selector {
            Selector::Eight => {
                ui.horizontal(|ui| {
                    for bit in self.eight.bits {
                        ui.label(bit.to_string());
                    }
                });

                if ui.button("Step").clicked() {
                    self.eight.step();
                }
            }
            Selector::Sixteen => {
                ui.horizontal(|ui| {
                    for bit in self.sixteen.bits {
                        ui.label(bit.to_string());
                    }
                });

                if ui.button("Step").clicked() {
                    self.sixteen.step();
                }
            }
            Selector::ThirtyTwo => {
                ui.horizontal(|ui| {
                    for bit in self.thirty_two.bits {
                        ui.label(bit.to_string());
                    }
                });

                if ui.button("Step").clicked() {
                    self.thirty_two.step();
                }
            }
        }
    }
}
