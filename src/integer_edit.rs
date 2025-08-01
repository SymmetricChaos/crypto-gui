#![allow(clippy::needless_pass_by_value)] // False positives with `impl ToString`

use egui::{
    text, Button, CursorIcon, Key, Modifiers, Response, RichText, Sense, TextEdit, TextWrapMode,
    Ui, Widget,
};
use num::Zero;
use paste::paste;
use std::{cmp::Ordering, ops::RangeInclusive};

pub enum IntegerFormatType {
    Hex,
    Dec,
}

// This is basically just the DragValue code with everything ripped out until I got just what I wanted
macro_rules! integer_edit_box {
    ($name: ident, $t: ty, $formatter: expr) => {
        paste! {
            fn [<parse_ $t >](text: &str) -> Option<$t> {
                let text: String = text
                    .chars()
                    // Ignore whitespace (trailing, leading, and thousands separators):
                    .filter(|c| !c.is_whitespace())
                    .collect();

                <$t>::from_str_radix(&text, 16).ok()
            }

            fn [<clamp $t >](x: $t, range: RangeInclusive<$t>) -> $t {
                let (mut min, mut max) = (*range.start(), *range.end());

                if min.cmp(&max) == Ordering::Greater {
                    (min, max) = (max, min);
                }

                match x.cmp(&min) {
                    Ordering::Less | Ordering::Equal => min,
                    Ordering::Greater => match x.cmp(&max) {
                        Ordering::Greater | Ordering::Equal => max,
                        Ordering::Less => x,
                    },
                }
            }

            #[allow(non_camel_case_types)]
            type [<get_set_ $t >]<'a> = Box<dyn 'a + FnMut(Option<$t>) -> $t>;

            fn [<get_ $t >](get_set_value: &mut [<get_set_ $t >]<'_>) -> $t {
                (get_set_value)(None)
            }

            fn [<set_ $t >](get_set_value: &mut [<get_set_ $t >]<'_>, value: $t) {
                (get_set_value)(Some(value));
            }

            #[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
            pub struct $name<'a> {
                get_set_value: [<get_set_ $t >]<'a>,
                speed: $t,
                prefix: String,
                suffix: String,
                range: RangeInclusive<$t>,
                clamp_to_range: bool,
                update_while_editing: bool,
                format_type: IntegerFormatType,
            }

            impl<'a> $name<'a> {
                pub fn new(value: &'a mut $t) -> Self {
                    let slf = Self::from_get_set(move |v: Option<$t>| {
                        if let Some(v) = v {
                            *value = v
                        }
                        *value
                    });

                    slf
                }

                pub fn from_get_set(get_set_value: impl 'a + FnMut(Option<$t>) -> $t) -> Self {
                    Self {
                        get_set_value: Box::new(get_set_value),
                        speed: 1,
                        prefix: Default::default(),
                        suffix: Default::default(),
                        range: <$t>::MIN..=<$t>::MAX,
                        clamp_to_range: true,
                        update_while_editing: false,
                        format_type: IntegerFormatType::Hex,
                    }
                }

                #[inline]
                pub fn format_type(mut self, format_type: IntegerFormatType) -> Self {
                    self.format_type = format_type;
                    self
                }

                #[inline]
                pub fn range(mut self, range: RangeInclusive<$t>) -> Self {
                    self.range = range;
                    self
                }

                #[inline]
                pub fn clamp_to_range(mut self, clamp_to_range: bool) -> Self {
                    self.clamp_to_range = clamp_to_range;
                    self
                }

                /// Show a prefix before the number, e.g. "x: "
                #[inline]
                pub fn prefix(mut self, prefix: impl ToString) -> Self {
                    self.prefix = prefix.to_string();
                    self
                }

                /// Add a suffix to the number, this can be e.g. a unit ("°" or " m")
                #[inline]
                pub fn suffix(mut self, suffix: impl ToString) -> Self {
                    self.suffix = suffix.to_string();
                    self
                }

                /// If `false`, the value will only be updated when user presses enter or deselects the value.
                #[inline]
                pub fn update_while_editing(mut self, update: bool) -> Self {
                    self.update_while_editing = update;
                    self
                }
            }

            impl<'a> Widget for $name<'a> {
                fn ui(self, ui: &mut Ui) -> Response {
                    let Self {
                        mut get_set_value,
                        speed,
                        range,
                        clamp_to_range,
                        prefix,
                        suffix,
                        update_while_editing,
                        format_type,
                    } = self;

                    let id = ui.next_auto_id();

                    // The following ensures that when a `DragValue` receives focus,
                    // it is immediately rendered in edit mode, rather than being rendered
                    // in button mode for just one frame. This is important for
                    // screen readers.
                    let is_kb_editing = ui.memory_mut(|mem| {
                        mem.interested_in_focus(id,ui.layer_id());
                        mem.has_focus(id)
                    });

                    // Doing this correctly require access to a private egui method
                    if ui.memory_mut(|mem| !mem.had_focus_last_frame(id) && mem.has_focus(id)) {
                        // if ui.memory_mut(|mem: &mut egui::Memory| mem.has_focus(id)) {
                        ui.data_mut(|data| data.remove::<String>(id));
                    }

                    let old_value = [<get_ $t >](&mut get_set_value);
                    let mut value = old_value;

                    let change = ui.input_mut(|input| {
                        let mut change = 0 as $t;

                        if is_kb_editing {
                            // This deliberately doesn't listen for left and right arrow keys,
                            // because when editing, these are used to move the caret.
                            // This behavior is consistent with other editable spinner/stepper
                            // implementations, such as Chromium's (for HTML5 number input).
                            // It is also normal for such controls to go directly into edit mode
                            // when they receive keyboard focus, and some screen readers
                            // assume this behavior, so having a separate mode for incrementing
                            // and decrementing, that supports all arrow keys, would be
                            // problematic.
                            change = change.wrapping_add(
                                input.count_and_consume_key(Modifiers::NONE, Key::ArrowUp) as $t,
                            );
                            change = change.wrapping_sub(
                                input.count_and_consume_key(Modifiers::NONE, Key::ArrowDown) as $t,
                            );
                        }

                        change
                    });


                    ui.input_mut(|input| {
                        if is_kb_editing {
                            // Simple step up and step down by one.
                            value = value.wrapping_add(
                                input.count_and_consume_key(Modifiers::NONE, Key::ArrowUp) as $t,
                            );
                            value = value.wrapping_sub(
                                input.count_and_consume_key(Modifiers::NONE, Key::ArrowDown) as $t,
                            );
                        }
                    });

                    if clamp_to_range {
                        value = [<clamp $t >](value, range.clone());
                    }

                    if change.is_zero() {
                        value = value.wrapping_add(change.wrapping_mul(speed));
                    }

                    if old_value != value {
                        [<set_ $t >](&mut get_set_value, value);
                        ui.data_mut(|data| data.remove::<String>(id));
                    }


                    let value_text = match format_type {
                        IntegerFormatType::Hex => format!($formatter, value),
                        IntegerFormatType::Dec => format!("{}", value),
                    };

                    let text_style = egui::TextStyle::Monospace;

                    if ui.memory(|mem| mem.had_focus_last_frame(id) && !mem.has_focus(id))
                        && !ui.input(|i| i.key_pressed(Key::Escape))
                    {
                        // if ui.memory(|mem| !mem.has_focus(id)) && !ui.input(|i| i.key_pressed(Key::Escape))
                        // {
                        let value_text = ui.data_mut(|data| data.remove_temp::<String>(id));
                        if let Some(value_text) = value_text {
                            // We were editing the value as text last frame, but lost focus.
                            // Make sure we applied the last text value:
                            let parsed_value = [<parse_ $t >](&value_text);
                            if let Some(mut parsed_value) = parsed_value {
                                if clamp_to_range {
                                    parsed_value = [<clamp $t >](parsed_value, range.clone());
                                }
                                [<set_ $t >](&mut get_set_value, parsed_value);
                            }
                        }
                    }

                    // some clones below are redundant if AccessKit is disabled
                    #[allow(clippy::redundant_clone)]
                    let response = if is_kb_editing {
                        let mut value_text = ui
                            .data_mut(|data| data.remove_temp::<String>(id))
                            .unwrap_or_else(|| value_text.clone());
                        let response = ui.add(
                            TextEdit::singleline(&mut value_text)
                                .clip_text(false)
                                .horizontal_align(ui.layout().horizontal_align())
                                .vertical_align(ui.layout().vertical_align())
                                .margin(ui.spacing().button_padding)
                                .min_size(ui.spacing().interact_size)
                                .id(id)
                                .desired_width(ui.spacing().interact_size.x)
                                .font(text_style),
                        );

                        let update = if update_while_editing {
                            // Update when the edit content has changed.
                            response.changed()
                        } else {
                            // Update only when the edit has lost focus.
                            response.lost_focus() && !ui.input(|i| i.key_pressed(Key::Escape))
                        };
                        if update {
                            let parsed_value = [<parse_ $t >](&value_text);
                            if let Some(mut parsed_value) = parsed_value {
                                if clamp_to_range {
                                    parsed_value = [<clamp $t >](parsed_value, range.clone());
                                }
                                [<set_ $t >](&mut get_set_value, parsed_value);
                            }
                        }
                        ui.data_mut(|data| data.insert_temp(id, value_text));
                        response
                    } else {
                        let button = Button::new(
                            RichText::new(format!("{}{}{}", prefix, value_text.clone(), suffix))
                                .text_style(text_style),
                        )
                        .wrap_mode(TextWrapMode::Extend)
                        .sense(Sense::click_and_drag())
                        .min_size(ui.spacing().interact_size); // TODO(emilk): find some more generic solution to `min_size`

                        let cursor_icon = CursorIcon::Text;

                        let response = ui.add(button);
                        let mut response = response.on_hover_cursor(cursor_icon);

                        if ui.style().explanation_tooltips {
                            response = response.on_hover_text(format!("Click to enter a value."));
                        }

                        if response.clicked() {
                            ui.data_mut(|data| data.remove::<String>(id));
                            ui.memory_mut(|mem| mem.request_focus(id));
                            let mut state = TextEdit::load_state(ui.ctx(), id).unwrap_or_default();
                            state.cursor.set_char_range(Some(text::CCursorRange::two(
                                text::CCursor::default(),
                                text::CCursor::new(value_text.chars().count()),
                            )));
                            state.store(ui.ctx(), response.id);
                        }

                        response
                    };

                    // response.sense.set([<get_ $t >](&mut get_set_value) != old_value);

                    response
                }
            }
        }
    };
}
integer_edit_box!(EditU8, u8, "{:0>2x}");
integer_edit_box!(EditU16, u16, "{:0>4x}");
integer_edit_box!(EditU32, u32, "{:0>8x}");
integer_edit_box!(EditU64, u64, "{:0>16x}");
integer_edit_box!(EditU128, u128, "{:0>32x}");
