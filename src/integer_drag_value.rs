#![allow(clippy::needless_pass_by_value)] // False positives with `impl ToString`

use std::{cmp::Ordering, ops::RangeInclusive};

use egui::{
    text, Button, CursorIcon, Key, Response, RichText, Sense, TextEdit, TextWrapMode, Ui, Widget,
};

// ----------------------------------------------------------------------------

// type NumFormatter<'a> = Box<dyn 'a + Fn(u64, RangeInclusive<usize>) -> String>;
// type NumParser<'a> = Box<dyn 'a + Fn(&str) -> Option<u64>>;

// ----------------------------------------------------------------------------

/// Combined into one function (rather than two) to make it easier
/// for the borrow checker.
type GetSetValue<'a> = Box<dyn 'a + FnMut(Option<u64>) -> u64>;

fn get(get_set_value: &mut GetSetValue<'_>) -> u64 {
    (get_set_value)(None)
}

fn set(get_set_value: &mut GetSetValue<'_>, value: u64) {
    (get_set_value)(Some(value));
}

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct EditU64<'a> {
    get_set_value: GetSetValue<'a>,
    prefix: String,
    suffix: String,
    range: RangeInclusive<u64>,
    clamp_to_range: bool,
    // custom_formatter: Option<NumFormatter<'a>>,
    // custom_parser: Option<NumParser<'a>>,
    update_while_editing: bool,
}

impl<'a> EditU64<'a> {
    pub fn new(value: &'a mut u64) -> Self {
        let slf = Self::from_get_set(move |v: Option<u64>| {
            if let Some(v) = v {
                *value = v
            }
            *value
        });

        slf
    }

    pub fn from_get_set(get_set_value: impl 'a + FnMut(Option<u64>) -> u64) -> Self {
        Self {
            get_set_value: Box::new(get_set_value),
            // speed: 1,
            prefix: Default::default(),
            suffix: Default::default(),
            range: u64::MIN..=u64::MAX,
            clamp_to_range: true,
            // custom_formatter: None,
            // custom_parser: None,
            update_while_editing: true,
        }
    }

    #[deprecated = "Use `range` instead"]
    #[inline]
    pub fn clamp_range(mut self, range: RangeInclusive<u64>) -> Self {
        self.range = range;
        self
    }

    #[inline]
    pub fn range(mut self, range: RangeInclusive<u64>) -> Self {
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

    // pub fn custom_formatter(
    //     mut self,
    //     formatter: impl 'a + Fn(u64, RangeInclusive<usize>) -> String,
    // ) -> Self {
    //     self.custom_formatter = Some(Box::new(formatter));
    //     self
    // }

    // #[inline]
    // pub fn custom_parser(mut self, parser: impl 'a + Fn(&str) -> Option<u64>) -> Self {
    //     self.custom_parser = Some(Box::new(parser));
    //     self
    // }

    // pub fn binary(self, min_width: usize) -> Self {
    //     assert!(
    //         min_width > 0,
    //         "DragValue::binary: `min_width` must be greater than 0"
    //     );
    //     self.custom_formatter(move |n, _| format!("{:0>min_width$b}", n))
    //         .custom_parser(|s| u64::from_str_radix(s, 2).ok())
    // }

    // pub fn octal(self, min_width: usize) -> Self {
    //     assert!(
    //         min_width > 0,
    //         "DragValue::octal: `min_width` must be greater than 0"
    //     );
    //     self.custom_formatter(move |n, _| format!("{:0>min_width$o}", n))
    //         .custom_parser(|s| u64::from_str_radix(s, 8).ok())
    // }

    // pub fn hexadecimal(self, min_width: usize, upper: bool) -> Self {
    //     assert!(
    //         min_width > 0,
    //         "DragValue::hexadecimal: `min_width` must be greater than 0"
    //     );
    //     match upper {
    //         true => self.custom_formatter(move |n, _| format!("{:0>min_width$X}", n)),
    //         false => self.custom_formatter(move |n, _| format!("{:0>min_width$x}", n)),
    //     }
    //     .custom_parser(|s| u64::from_str_radix(s, 16).ok())
    // }

    /// Update the value on each key press when text-editing the value.
    ///
    /// Default: `true`.
    /// If `false`, the value will only be updated when user presses enter or deselects the value.
    #[inline]
    pub fn update_while_editing(mut self, update: bool) -> Self {
        self.update_while_editing = update;
        self
    }
}

impl<'a> Widget for EditU64<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            mut get_set_value,
            // speed,
            range,
            clamp_to_range,
            prefix,
            suffix,
            // min_decimals,
            // max_decimals,
            // custom_formatter,
            // custom_parser,
            update_while_editing,
        } = self;

        let id = ui.next_auto_id();

        // The following ensures that when a `DragValue` receives focus,
        // it is immediately rendered in edit mode, rather than being rendered
        // in button mode for just one frame. This is important for
        // screen readers.
        let is_kb_editing = ui.memory_mut(|mem| {
            mem.interested_in_focus(id);
            mem.has_focus(id)
        });

        // This require access to a private egui method
        // if ui.memory_mut(|mem| mem.gained_focus(id)) {
        if ui.memory_mut(|mem: &mut egui::Memory| mem.has_focus(id)) {
            ui.data_mut(|data| data.remove::<String>(id));
        }

        let old_value = get(&mut get_set_value);
        let mut value = old_value;

        // let change = ui.input_mut(|input| {
        //     let mut change = 0_u64;

        //     if is_kb_editing {
        //         // This deliberately doesn't listen for left and right arrow keys,
        //         // because when editing, these are used to move the caret.
        //         // This behavior is consistent with other editable spinner/stepper
        //         // implementations, such as Chromium's (for HTML5 number input).
        //         // It is also normal for such controls to go directly into edit mode
        //         // when they receive keyboard focus, and some screen readers
        //         // assume this behavior, so having a separate mode for incrementing
        //         // and decrementing, that supports all arrow keys, would be
        //         // problematic.
        //         change += input.count_and_consume_key(Modifiers::NONE, Key::ArrowUp) as u64
        //             - input.count_and_consume_key(Modifiers::NONE, Key::ArrowDown) as u64;
        //     }
        // });

        if clamp_to_range {
            value = clamp_value_to_range(value, range.clone());
        }

        if old_value != value {
            set(&mut get_set_value, value);
            ui.data_mut(|data| data.remove::<String>(id));
        }

        let value_text = format!("{:016x?}", value);

        let text_style = egui::TextStyle::Monospace;

        // if ui.memory(|mem| mem.lost_focus(id)) && !ui.input(|i| i.key_pressed(Key::Escape)) {
        if ui.memory(|mem| !mem.has_focus(id)) && !ui.input(|i| i.key_pressed(Key::Escape)) {
            let value_text = ui.data_mut(|data| data.remove_temp::<String>(id));
            if let Some(value_text) = value_text {
                // We were editing the value as text last frame, but lost focus.
                // Make sure we applied the last text value:
                let parsed_value = parse(&value_text);
                if let Some(mut parsed_value) = parsed_value {
                    if clamp_to_range {
                        parsed_value = clamp_value_to_range(parsed_value, range.clone());
                    }
                    set(&mut get_set_value, parsed_value);
                }
            }
        }

        // some clones below are redundant if AccessKit is disabled
        #[allow(clippy::redundant_clone)]
        let mut response = if is_kb_editing {
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
                let parsed_value = parse(&value_text);
                if let Some(mut parsed_value) = parsed_value {
                    if clamp_to_range {
                        parsed_value = clamp_value_to_range(parsed_value, range.clone());
                    }
                    set(&mut get_set_value, parsed_value);
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
            //  if value <= *range.start() {
            //     CursorIcon::ResizeEast
            // } else if value < *range.end() {
            //     CursorIcon::ResizeHorizontal
            // } else {
            //     CursorIcon::ResizeWest
            // };

            let response = ui.add(button);
            let mut response = response.on_hover_cursor(cursor_icon);

            if ui.style().explanation_tooltips {
                response = response.on_hover_text(format!(
                    "{}{}{}\nClick to enter a value.",
                    prefix, value as u64, suffix
                ));
            }

            // if ui.input(|i| i.pointer.any_pressed() || i.pointer.any_released()) {
            //     // Reset memory of preciely dagged value.
            //     ui.data_mut(|data| data.remove::<f64>(id));
            // }

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

        response.changed = get(&mut get_set_value) != old_value;

        // response.widget_info(|| WidgetInfo::drag_value(ui.is_enabled(), value));

        response
    }
}

fn parse(value_text: &str) -> Option<u64> {
    u64::from_str_radix(value_text, 16).ok()
}

fn clamp_value_to_range(x: u64, range: RangeInclusive<u64>) -> u64 {
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