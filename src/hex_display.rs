use egui::{Response, TextStyle};
use utils::text_functions::filter_string;

pub fn u64_zeroes() -> String {
    String::from("0000000000000000")
}

pub fn control_hex_u64(ui: &mut egui::Ui, string: &mut String, value: &mut u64) -> Response {
    let resp = ui.add(
        egui::TextEdit::singleline(string)
            .font(TextStyle::Monospace)
            .clip_text(false)
            .min_size(ui.spacing().interact_size)
            .desired_width(ui.spacing().interact_size.x),
    );
    // After clicking off immediately recalculate the value from the text
    if resp.lost_focus() {
        filter_string(string, &"0123456789ABCDEFabcdef");
        if string.is_empty() {
            *string = u64_zeroes();
            *value = 0;
        }
        match u64::from_str_radix(string, 16) {
            Ok(b) => *value = b,
            Err(_) => {
                *value = u64::MAX;
                *string = String::from("ffffffffffffffff")
            }
        };
    }
    // Whenever we don't have focus rewrite the text from the value
    if !resp.has_focus() {
        *string = format!("{:016x?}", value)
    }
    resp
}
