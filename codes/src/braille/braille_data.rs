// The 64 possible Braille cells as organized by UEB specification, excluding the space
// Unicode Braille space: "⠀" <- right there
pub const LINE1: &'static str = "⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚";
pub const LINE2: &'static str = "⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞";
pub const LINE3: &'static str = "⠥⠧⠭⠽⠵⠯⠿⠷⠮⠾";
pub const LINE4: &'static str = "⠡⠣⠩⠹⠱⠫⠻⠳⠪⠺";
pub const LINE5: &'static str = "⠂⠆⠒⠲⠢⠖⠶⠦⠔⠴";
pub const LINE6: &'static str = "⠌⠬⠼⠜⠄⠤";
pub const LINE7: &'static str = "⠈⠘⠸⠐⠨⠰⠠";

pub const UEB_ROWS: [&'static str; 7] = [LINE1, LINE2, LINE3, LINE4, LINE5, LINE6, LINE7];

pub const UEB_ORDER: &'static str =
    "⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠭⠽⠵⠯⠿⠷⠮⠾⠡⠣⠩⠹⠱⠫⠻⠳⠪⠺⠂⠆⠒⠲⠢⠖⠶⠦⠔⠴⠌⠬⠼⠜⠄⠤⠈⠘⠸⠐⠨⠰⠠";

// Unicode orders Braille cells by assigning a numeric value to each dot
pub const UNICODE_ORDER: &'static str =
    "⠁⠂⠃⠄⠅⠆⠇⠈⠉⠊⠋⠌⠍⠎⠏⠐⠑⠒⠓⠔⠕⠖⠗⠘⠙⠚⠛⠜⠝⠞⠟⠠⠡⠢⠣⠤⠥⠦⠧⠨⠩⠪⠫⠬⠭⠮⠯⠰⠱⠲⠳⠴⠵⠶⠷⠸⠹⠺⠻⠼⠽⠾⠿";

// These eight characters are the UEB prefixes. All others characters are called roots as is the space.
pub const PREFIXES: &'static str = "⠼⠈⠘⠸⠐⠨⠰⠠";
