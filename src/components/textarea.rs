use std::{io::Write, str::Chars};

use anathema::{
    default_widgets::{Overflow, Text},
    geometry::Pos,
    prelude::Context,
    state::{Number, State, Value},
    widgets::{layout::text::Segment, Elements},
};

pub const TEXTAREA_TEMPLATE: &str = "./src/components/templates/textarea.aml";

#[derive(Default)]
pub struct TextArea;

#[derive(Default, anathema::state::State)]
pub struct TextAreaInputState {
    log_output: Value<String>,
    input: Value<String>,
    cursor_prefix: Value<String>,
    cursor_char: Value<String>,
    cursor_position: Value<Coordinates>,
    line_count: Value<usize>,
    fg_color: Value<String>,
    bg_color: Value<String>,
    focused: Value<bool>,
}

#[derive(Default, anathema::state::State)]
struct Coordinates {
    x: Value<usize>,
    y: Value<usize>,
}

impl Coordinates {
    pub fn new(x_pos: usize, y_pos: usize) -> Self {
        Coordinates {
            x: x_pos.into(),
            y: y_pos.into(),
        }
    }
}

impl TextAreaInputState {
    pub fn new() -> Self {
        TextAreaInputState {
            input: String::from("").into(),
            cursor_prefix: String::from("").into(),
            cursor_char: String::from("").into(),
            cursor_position: Coordinates::new(0, 0).into(),
            line_count: 1.into(),
            fg_color: String::from("white").into(),
            bg_color: String::from("").into(),
            focused: false.into(),
            log_output: String::from("").into(),
        }
    }
}

impl anathema::component::Component for TextArea {
    type State = TextAreaInputState;
    type Message = ();

    fn on_focus(
        &mut self,
        state: &mut Self::State,
        _: Elements<'_, '_>,
        _: Context<'_, Self::State>,
    ) {
        let input = state.input.to_ref();
        // let Some(cursor_position) = state.cursor_position.to_number() else {
        //     return;
        // };
        // let pos = cursor_position.as_uint();
        //
        // let cursor_char = if pos == input.len() {
        //     ' '
        // } else {
        //     input.chars().nth(pos).unwrap_or(' ')
        // };

        // state.cursor_char.set(cursor_char.to_string());
        state.fg_color.set("black".to_string());
        state.bg_color.set("white".to_string());
        state.focused.set(true);
    }

    fn on_blur(
        &mut self,
        state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_, Self::State>,
    ) {
        state.cursor_char.set("".to_string());
        state.fg_color.set("white".to_string());
        state.bg_color.set("".to_string());
        state.focused.set(false);
    }

    fn on_key(
        &mut self,
        event: anathema::component::KeyEvent,
        state: &mut Self::State,
        elements: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        // let mut input = state.input.to_mut();
        if !*state.focused.to_ref() {
            return;
        }

        match event.code {
            // NOTE: Unused for TextInput
            // anathema::component::KeyCode::Home => todo!(),
            // anathema::component::KeyCode::End => todo!(),
            // anathema::component::KeyCode::PageUp => todo!(),
            // anathema::component::KeyCode::PageDown => todo!(),
            // anathema::component::KeyCode::F(_) => todo!(),
            // anathema::component::KeyCode::Null => todo!(),
            // anathema::component::KeyCode::CapsLock => todo!(),
            // anathema::component::KeyCode::ScrollLock => todo!(),
            // anathema::component::KeyCode::NumLock => todo!(),
            // anathema::component::KeyCode::PrintScreen => todo!(),
            // anathema::component::KeyCode::Pause => todo!(),
            // anathema::component::KeyCode::Menu => todo!(),
            // anathema::component::KeyCode::KeypadBegin => todo!(),

            // Text Input events

            // TODO: Ask togglebit Discord if I'm supposed to get this key event
            anathema::component::KeyCode::Tab => {
                let char = '\u{0009}'; // Tab
                self.add_character(char, state, context, elements);
            }

            anathema::component::KeyCode::Char(char) => {
                self.add_character(char, state, context, elements)
            }
            anathema::component::KeyCode::Backspace => self.backspace(state, context),
            anathema::component::KeyCode::Delete => self.delete(state, context),
            anathema::component::KeyCode::Left => self.move_cursor_left(state, elements),
            anathema::component::KeyCode::Right => self.move_cursor_right(state, elements),
            anathema::component::KeyCode::Up => self.move_cursor_up(state),
            anathema::component::KeyCode::Down => self.move_cursor_down(state),

            // TODO: This will need to call some callback or something?
            anathema::component::KeyCode::Enter => {
                let char = '\u{000A}';
                self.add_character(char, state, context, elements)
            }

            // TODO: Maybe I'll implement this later
            anathema::component::KeyCode::Insert => todo!(),

            // TODO: Ask togglebit Discord if I'm supposed to get this key event
            anathema::component::KeyCode::BackTab => todo!(),

            // TODO: Maybe implement this later, this will require implementing selections in the
            // input
            anathema::component::KeyCode::CtrlC => todo!(),

            // Move focus with this
            anathema::component::KeyCode::Esc => context.set_focus("id", "app"),

            _ => {}
        }
    }
}

fn log(output: String, file: Option<&str>) {
    let file_name = file.unwrap_or("logs.txt");
    let file = std::fs::OpenOptions::new().append(true).open(file_name);
    if let Ok(mut file) = file {
        let _ = file.write(output.as_bytes());
    }
}

fn update_cursor_char(input: &mut Chars, update_index: usize) -> String {
    if let Some(cursor_char) = input.nth(update_index) {
        cursor_char.to_string()
    } else {
        " ".to_string()
    }
}

impl TextArea {
    fn add_character(
        &mut self,
        char: char,
        state: &mut TextAreaInputState,
        mut context: Context<'_, TextAreaInputState>,
        mut elements: anathema::widgets::Elements<'_, '_>,
    ) {
        let mut input = state.input.to_mut();

        elements
            .by_attribute("id", "contents")
            .each(|el, _attributes| {
                let text = el.to::<Text>();

                // Update line count
                let current_line_count = text.get_line_count();
                let prev_line_count = state
                    .line_count
                    .to_number()
                    .unwrap_or(Number::Usize(0))
                    .as_uint();
                state.line_count.set(current_line_count);

                // Get coordinates before this character input
                let mut cursor_coordinates = state.cursor_position.to_mut();
                let prev_cursor_x = cursor_coordinates
                    .x
                    .to_number()
                    .unwrap_or(Number::Usize(0))
                    .as_uint();
                let prev_cursor_y = cursor_coordinates
                    .y
                    .to_number()
                    .unwrap_or(Number::Usize(0))
                    .as_uint();

                log(format!("prev_x: {prev_cursor_x}\n"), None);
                log(format!("prev_y: {prev_cursor_y}\n"), None);

                // Calculate new cursor X/Y position
                let new_cursor_x = if current_line_count > prev_line_count {
                    Number::Usize(0).as_uint()
                } else {
                    Number::Usize(prev_cursor_x + 1).as_uint()
                };
                cursor_coordinates.x.set(new_cursor_x);

                let new_cursor_y = if current_line_count > prev_line_count {
                    Number::Usize(prev_cursor_y + 1).as_uint()
                } else {
                    Number::Usize(prev_cursor_y).as_uint()
                };
                cursor_coordinates.y.set(new_cursor_y);

                log(format!("new_x: {new_cursor_x}\n"), None);
                log(format!("new_y: {new_cursor_y}\n"), None);

                // Get line lengths for all lines in input
                let lines = text.get_lines();
                let mut line_lengths: Vec<usize> = [].to_vec();
                lines.enumerate().for_each(|(index, current_line)| {
                    log(format!("Looking at line_number: {index}\n"), None);

                    let mut length_of_line = 0;
                    current_line.entries.for_each(|entry| {
                        log(format!("Checking line entry for line {index}\n"), None);

                        if let Segment::Str(text) = entry {
                            let chunk_length = text.len();

                            log(format!("Chunk: {text}, chunk_length: {chunk_length}\n"), None);

                            length_of_line += chunk_length;
                        };
                    });


                    log(format!("Length of line {index}: {length_of_line}\n"), None);

                    line_lengths.push(length_of_line);
                });

                // Account for newline characters, but remove one because the last line doesn't
                // have a newline character
                line_lengths.push(line_lengths.len() - 1);

                log(format!("Length of all lines: {}\n", line_lengths.iter().sum::<usize>()), None);

                // Sets update index to the end of the input string
                let update_index = line_lengths.iter().sum::<usize>();
                log(format!("Initial update_index: {update_index}\n"), None);


                // NOTE: Adjust update_index when on a new line, commented out because linear editing
                // works, this might be needed, with updated logic, once the cursor starts to move
                // around and the editing is no longer linear
                //
                // if new_cursor_y > prev_cursor_y && new_cursor_y > 1 {
                //     log("We are on a new line, resetting update_index to sum of line lengths\n".to_string());
                //
                //     let lengths_iter = line_lengths.iter();
                //
                //     let previous_lines_sum = lengths_iter.sum::<usize>();
                //     log(format!("previous_lines_sum: {previous_lines_sum}\n"));
                //
                //     update_index = previous_lines_sum + new_cursor_x;
                //
                //     log(format!("update_index when going to a next line: {update_index}\n"));
                // }

                log(format!("Final update_index: {update_index}\n"), None);
                log(format!("input.len(): {}\n", input.len()), None);
                log(format!(
                    "Inserting at index: {update_index} character: {char}, current input length: {} \n", input.len()
                ), None);
                log(
                    "----------------------------------------\n".to_string(), None
                );

                // Insert new character
                input.insert(update_index, char);

                // Update text prefix
                let prefix_text = input.chars().take(update_index + 1).collect::<String>();
                state.cursor_prefix.set(prefix_text);

                let mut chars = input.chars();
                let cursor_char = update_cursor_char(&mut chars, update_index + 1);
                state.cursor_char.set(cursor_char);
            });

        elements
            .by_attribute("id", "container")
            .each(|el, _attributes| {
                let coordinates = state.cursor_position.to_ref();
                let x = coordinates.x.to_number().unwrap_or(Number::I32(0)).as_int() as i32;
                let y = coordinates.y.to_number().unwrap_or(Number::I32(0)).as_int() as i32;

                let position = Pos { x, y };
                let overflow = el.to::<Overflow>();
                overflow.scroll_to(position);
            });

        context.publish("text_change", |state| &state.input)
    }

    fn delete(&self, state: &mut TextAreaInputState, mut context: Context<'_, TextAreaInputState>) {
        // let mut input = state.input.to_mut();
        // let Some(cursor_position) = state.cursor_position.to_number() else {
        //     return;
        // };
        //
        // let pos = cursor_position.as_uint();
        //
        // if pos == input.len() {
        //     return;
        // }
        //
        // input.remove(pos);
        //
        // if let Some(cursor_char) = input.chars().nth(pos) {
        //     state.cursor_char.set(cursor_char.to_string());
        // } else {
        //     state.cursor_char.set(' '.to_string());
        // }
        //
        // state
        //     .cursor_prefix
        //     .set(input.chars().take(pos).collect::<String>());
        //
        // context.publish("text_change", |state| &state.input)
    }

    fn backspace(
        &mut self,
        state: &mut TextAreaInputState,
        mut context: Context<'_, TextAreaInputState>,
    ) {
        // let mut input = state.input.to_mut();
        // let Some(cursor_position) = state.cursor_position.to_number() else {
        //     return;
        // };
        //
        // let pos = cursor_position.as_uint();
        //
        // if pos == 0 {
        //     return;
        // }
        //
        // let new_pos = pos - 1;
        // input.remove(new_pos);
        //
        // state.cursor_position.set(new_pos);
        //
        // if let Some(cursor_char) = input.chars().nth(new_pos) {
        //     state.cursor_char.set(cursor_char.to_string());
        // } else {
        //     state.cursor_char.set(' '.to_string());
        // }
        //
        // state
        //     .cursor_prefix
        //     .set(input.chars().take(new_pos).collect::<String>());
        //
        // context.publish("text_change", |state| &state.input)
    }

    fn move_cursor_left(
        &self,
        state: &mut TextAreaInputState,
        mut elements: anathema::widgets::Elements<'_, '_>,
    ) {
        // TODO: Refactor this so its not repeated when moving right
        let cursor_index = state.cursor_prefix.to_ref().len();
        let new_cursor_index = cursor_index.saturating_sub(1);
        let current_input = state.input.to_ref();
        let new_prefix = current_input.chars().take(new_cursor_index);
        state.cursor_prefix.set(new_prefix.collect::<String>());

        let input = state.input.to_ref();
        let mut chars = input.chars();
        let cursor_char = update_cursor_char(&mut chars, new_cursor_index);
        state.cursor_char.set(cursor_char);

        // Update cursor x/y position
        let mut coordinates = state.cursor_position.to_mut();
        let mut x = *coordinates.x.to_ref();
        let mut y = *coordinates.y.to_ref();

        if x > 0 {
            coordinates.x.set(x - 1);
        } else {
            elements
                .by_attribute("id", "contents")
                .each(|el, _attributes| {
                    let text = el.to::<Text>();
                    let mut lines = text.get_lines();

                    y = y.saturating_sub(1);
                    if let Some(previous_line) = lines.nth(y) {
                        x = previous_line.width as usize + 1;
                    }
                });

            coordinates.x.set(x);
            coordinates.y.set(y);
        }
    }

    fn move_cursor_right(
        &self,
        mut state: &mut TextAreaInputState,
        mut elements: anathema::widgets::Elements<'_, '_>,
    ) {
        elements
            .by_attribute("id", "contents")
            .each(|el, _attributes| {
                // TODO: Fix this clone
                let prefix = state.cursor_prefix.to_ref().clone();
                let input = state.input.to_ref();
                if prefix.len() == input.len() {
                    // At the end, can't move to the right
                    return;
                }

                let text = el.to::<Text>();
                let mut lines = text.get_lines();

                let mut coordinates = state.cursor_position.to_mut();
                let x = *coordinates.x.to_ref();
                let y = *coordinates.y.to_ref();

                let Some(current_line) = lines.nth(y) else {
                    return;
                };

                let current_input = state.input.to_ref();
                let last_current_line_x = current_line.width + 1;

                match x == last_current_line_x.into() {
                    true => {
                        coordinates.x.set(0);
                        coordinates.y.set(y + 1);
                    }
                    false => {
                        coordinates.x.set(x + 1);
                    }
                }

                let cursor_index = prefix.len();
                let new_cursor_index = cursor_index + 1;
                let new_prefix = current_input.chars().take(new_cursor_index);
                state.cursor_prefix.set(new_prefix.collect::<String>());

                // let input = state.input.to_ref();
                let mut chars = current_input.chars();
                let cursor_char = update_cursor_char(&mut chars, new_cursor_index);
                state.cursor_char.set(cursor_char);

                log(
                    format!(
                        "x: {}, y: {}\n",
                        *coordinates.x.to_ref(),
                        *coordinates.y.to_ref()
                    ),
                    Some("move_right.txt"),
                );
            });
    }

    fn move_cursor_up(&self, state: &mut TextAreaInputState) {
        // let input = state.input.to_mut();
        //
        // state.cursor_position.set(input.len());
        // state.cursor_char.set(' '.to_string());
        // state.cursor_prefix.set(input.to_string());
    }

    fn move_cursor_down(&self, state: &mut TextAreaInputState) {
        // let input = state.input.to_mut();
        //
        // state.cursor_position.set(0);
        // state
        //     .cursor_char
        //     .set(input.chars().nth(0).unwrap_or(' ').to_string());
        // state.cursor_prefix.set("".to_string());
    }
}
