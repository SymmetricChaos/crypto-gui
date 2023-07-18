use super::CipherFrame;
use crate::ui_elements::UiElements;
use ciphers::{
    machines::m209::{M209, M209_ALPHABETS},
    Cipher,
};
use egui::{Slider, Ui};
use rand::{thread_rng, Fill};
use utils::{
    preset_alphabet::Alphabet,
    text_functions::{filter_string, random_sample},
};

fn lug_pair(ui: &mut egui::Ui, pair: &mut (usize, usize)) {
    ui.add(
        egui::DragValue::new(&mut pair.0)
            .clamp_range(0usize..=6)
            .speed(0.1),
    );
    ui.add(
        egui::DragValue::new(&mut pair.1)
            .clamp_range(0usize..=6)
            .speed(0.1),
    );
}

#[derive(Default)]
pub struct M209Frame {
    cipher: M209,
    effective_pins: [String; 6],
}

impl CipherFrame for M209Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        ui.randomize_reset(self);
        ui.add_space(8.0);

        ui.subheading("Alphabet");
        ui.false_control_string(Alphabet::BasicLatin);
        ui.add_space(8.0);

        ui.subheading("Rotors");
        for (n, (rotor, pins)) in self
            .cipher
            .get_wheels()
            .zip(&mut self.effective_pins)
            .enumerate()
        {
            let len = rotor.rotor_length() - 1;
            ui.add(Slider::new(&mut rotor.active, 0..=len).show_value(false));
            ui.mono(&rotor);
            if ui.control_string(pins).changed() {
                filter_string(pins, M209_ALPHABETS[n]);
                rotor
                    .set_pins(pins)
                    .expect("filtering should prevent invalid pins from being reached");
            };

            ui.add_space(8.0);
        }
        ui.add_space(16.0);

        let lugs = &mut self.cipher.lugs;
        ui.subheading("Lug Pairs");
        for triple in lugs.chunks_exact_mut(3) {
            ui.horizontal(|ui| {
                lug_pair(ui, &mut triple[0]);
                ui.add_space(10.0);
                lug_pair(ui, &mut triple[1]);
                ui.add_space(10.0);
                lug_pair(ui, &mut triple[2]);
            });
            ui.add_space(10.0);
        }
    }

    fn cipher(&self) -> &dyn Cipher {
        &self.cipher
    }

    fn randomize(&mut self) {
        // Fill up an array with random bytes. Then map that to pairs of usize.
        // Unwrap here is justified by the fixed sizes of everything involved.
        let mut rng = thread_rng();
        let mut data = [0u8; 54];
        data.try_fill(&mut rng).unwrap();
        self.cipher.lugs = data
            .chunks_exact(2)
            .map(|x| ((x[0] % 7) as usize, (x[1] % 7) as usize))
            .collect::<Vec<(usize, usize)>>()
            .try_into()
            .unwrap();

        for ((pins, alphabet), rotor) in self
            .effective_pins
            .iter_mut()
            .zip(M209_ALPHABETS.iter())
            .zip(self.cipher.get_wheels())
        {
            *pins = random_sample(alphabet, 12, &mut rng);
            rotor
                .set_pins(pins)
                .expect("random pins should be drawn only from valid alphabets");
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
