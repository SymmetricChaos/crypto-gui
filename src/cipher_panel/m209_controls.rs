use ciphers::{Cipher, M209};
use egui::{Slider, Ui};
use rand::{thread_rng, Fill};
use utils::functions::random_char_vec;

use super::{CipherFrame, _generic_components::randomize_reset};

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
}

impl CipherFrame for M209Frame {
    fn ui(&mut self, ui: &mut Ui, _errors: &mut String) {
        randomize_reset(ui, self);
        ui.add_space(16.0);

        ui.label("Alphabet");
        ui.label("ABDCEFGHIJKLMNOPQRSTUVWXYZ");
        ui.add_space(16.0);

        ui.label("Rotor Settings");
        for rotor in self.cipher.get_wheels() {
            let len = rotor.rotor_length() - 1;
            ui.add(Slider::new(&mut rotor.active, 0..=len).show_value(false));
            ui.label(format!("{}", rotor));
        }

        ui.add_space(16.0);

        let lugs = &mut self.cipher.lugs;
        ui.label("Lugs");
        for triple in lugs.chunks_exact_mut(3) {
            ui.horizontal(|ui| {
                lug_pair(ui, &mut triple[0]);
                ui.add_space(4.0);
                lug_pair(ui, &mut triple[1]);
                ui.add_space(4.0);
                lug_pair(ui, &mut triple[2]);
            });
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

        let pins1 = random_char_vec("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 13, &mut rng);
        let pins2 = random_char_vec("ABCDEFGHIJKLMNOPQRSTUVXYZ", 12, &mut rng);
        let pins3 = random_char_vec("ABCDEFGHIJKLMNOPQRSTUVX", 12, &mut rng);
        let pins4 = random_char_vec("ABCDEFGHIJKLMNOPQRSTU", 12, &mut rng);
        let pins5 = random_char_vec("ABCDEFGHIJKLMNOPQRS", 12, &mut rng);
        let pins6 = random_char_vec("ABCDEFGHIJKLMNOPQ", 12, &mut rng);

        for (rotor, new_pins) in self
            .cipher
            .get_wheels()
            .zip([pins1, pins2, pins3, pins4, pins5, pins6].iter())
        {
            rotor.pins = new_pins.clone()
        }
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}
