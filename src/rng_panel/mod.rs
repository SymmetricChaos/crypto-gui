use egui::Ui;

mod fib_lfsr_controls;

pub trait View {
    fn ui(&mut self, ui: &mut Ui, errors: &mut String);
}
