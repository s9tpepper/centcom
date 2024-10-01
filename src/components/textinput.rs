use anathema::{
    prelude::Context,
    state::{AnyState, Value},
    widgets::Elements,
};

pub const TEXTINPUT_TEMPLATE: &str = "./src/components/templates/textinput.aml";

#[derive(Default)]
pub struct TextInput;

#[derive(Default, anathema::state::State)]
pub struct InputState {
    input: Value<String>,
    cursor_prefix: Value<String>,
    cursor_char: Value<String>,
    cursor_position: Value<usize>,
    fg_color: Value<String>,
    bg_color: Value<String>,
    focused: Value<bool>,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            input: String::from("").into(),
            cursor_prefix: String::from("").into(),
            cursor_char: String::from("").into(),
            cursor_position: 0.into(),
            fg_color: String::from("white").into(),
            bg_color: String::from("").into(),
            focused: false.into(),
        }
    }
}

impl anathema::component::Component for TextInput {
    type State = InputState;
    type Message = ();

    fn on_focus(
        &mut self,
        state: &mut Self::State,
        _: Elements<'_, '_>,
        _: Context<'_, Self::State>,
    ) {
        let input = state.input.to_ref();
        let Some(cursor_position) = state.cursor_position.to_number() else {
            return;
        };
        let pos = cursor_position.as_uint();

        let cursor_char = if pos == input.len() {
            ' '
        } else {
            input.chars().nth(pos).unwrap_or(' ')
        };

        state.cursor_char.set(cursor_char.to_string());
        state.fg_color.set("black".to_string());
        state.bg_color.set("white".to_string());
        state.focused.set(true);
    }

    fn on_blur(
        &mut self,
        state: &mut Self::State,
        _elements: Elements<'_, '_>,
        mut context: Context<'_, Self::State>,
    ) {
        state.cursor_char.set("".to_string());
        state.fg_color.set("white".to_string());
        state.bg_color.set("".to_string());
        state.focused.set(false);

        context.publish("textarea_focus", |state| &state.focused);
    }

    fn on_key(
        &mut self,
        event: anathema::component::KeyEvent,
        state: &mut Self::State,
        _: anathema::widgets::Elements<'_, '_>,
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
                self.add_character(char, state, context);
            }

            anathema::component::KeyCode::Char(char) => self.add_character(char, state, context),
            anathema::component::KeyCode::Backspace => self.backspace(state, context),
            anathema::component::KeyCode::Delete => self.delete(state, context),
            anathema::component::KeyCode::Left => self.move_cursor_left(state),
            anathema::component::KeyCode::Right => self.move_cursor_right(state),
            anathema::component::KeyCode::Up => self.move_cursor_up(state),
            anathema::component::KeyCode::Down => self.move_cursor_down(state),

            // TODO: This will need to call some callback or something?
            anathema::component::KeyCode::Enter => todo!(),

            // TODO: Maybe I'll implement this later
            anathema::component::KeyCode::Insert => todo!(),

            // TODO: Ask togglebit Discord if I'm supposed to get this key event
            anathema::component::KeyCode::BackTab => todo!(),

            // TODO: Maybe implement this later, this will require implementing selections in the
            // input
            anathema::component::KeyCode::CtrlC => todo!(),

            // Move focus with this
            anathema::component::KeyCode::Esc => {
                context.set_focus("id", "app");

                context.publish("textarea_focus", |state| &state.focused);
            }

            _ => {}
        }
    }
}

impl TextInput {
    fn add_character(
        &mut self,
        char: char,
        state: &mut InputState,
        mut context: Context<'_, InputState>,
    ) {
        let mut input = state.input.to_mut();
        let Some(cursor_position) = state.cursor_position.to_number() else {
            return;
        };

        // NOTE: Input when cursor is at the far right
        let pos = cursor_position.as_uint();
        input.insert(pos, char);

        let new_position = pos + 1;
        state.cursor_position.set(new_position);

        if pos == input.len() {
            state.cursor_prefix.set(input.to_string());
        } else {
            state
                .cursor_prefix
                .set(input.chars().take(new_position).collect::<String>())
        };

        let cursor_char = if pos == input.len() {
            ' '
        } else {
            input.chars().nth(new_position).unwrap_or(' ')
        };

        state.cursor_char.set(cursor_char.to_string());

        context.publish("text_change", |state| &state.input)
    }

    fn delete(&self, state: &mut InputState, mut context: Context<'_, InputState>) {
        let mut input = state.input.to_mut();
        let Some(cursor_position) = state.cursor_position.to_number() else {
            return;
        };

        let pos = cursor_position.as_uint();

        if pos == input.len() {
            return;
        }

        input.remove(pos);

        if let Some(cursor_char) = input.chars().nth(pos) {
            state.cursor_char.set(cursor_char.to_string());
        } else {
            state.cursor_char.set(' '.to_string());
        }

        state
            .cursor_prefix
            .set(input.chars().take(pos).collect::<String>());

        context.publish("text_change", |state| &state.input)
    }

    fn backspace(&mut self, state: &mut InputState, mut context: Context<'_, InputState>) {
        let mut input = state.input.to_mut();
        let Some(cursor_position) = state.cursor_position.to_number() else {
            return;
        };

        let pos = cursor_position.as_uint();

        if pos == 0 {
            return;
        }

        let new_pos = pos - 1;
        input.remove(new_pos);

        state.cursor_position.set(new_pos);

        if let Some(cursor_char) = input.chars().nth(new_pos) {
            state.cursor_char.set(cursor_char.to_string());
        } else {
            state.cursor_char.set(' '.to_string());
        }

        state
            .cursor_prefix
            .set(input.chars().take(new_pos).collect::<String>());

        context.publish("text_change", |state| &state.input)
    }

    fn move_cursor_left(&self, state: &mut InputState) {
        let input = state.input.to_mut();
        let Some(cursor_position) = state.cursor_position.to_number() else {
            return;
        };

        let pos = cursor_position.as_uint();
        if pos == 0 {
            return;
        }

        let new_pos = pos - 1;
        if let Some(new_char) = input.get(0..new_pos) {
            state.cursor_position.set(new_pos);
            state.cursor_prefix.set(new_char.to_string());

            if let Some(cursor_char) = input.to_string().chars().nth(new_pos) {
                state.cursor_char.set(cursor_char.to_string());
            }
        }
    }

    fn move_cursor_right(&self, state: &mut InputState) {
        let input = state.input.to_mut();
        let Some(cursor_position) = state.cursor_position.to_number() else {
            return;
        };

        let pos = cursor_position.as_uint();
        if pos == input.len() {
            return;
        }

        let new_pos = pos + 1;
        if let Some(new_char) = input.get(0..new_pos) {
            state.cursor_position.set(new_pos);
            state.cursor_prefix.set(new_char.to_string());

            if new_pos == input.len() {
                state.cursor_char.set(" ".to_string());
            } else if let Some(cursor_char) = input.to_string().chars().nth(new_pos) {
                state.cursor_char.set(cursor_char.to_string());
            }
        }
    }

    fn move_cursor_up(&self, state: &mut InputState) {
        let input = state.input.to_mut();

        state.cursor_position.set(input.len());
        state.cursor_char.set(' '.to_string());
        state.cursor_prefix.set(input.to_string());
    }

    fn move_cursor_down(&self, state: &mut InputState) {
        let input = state.input.to_mut();

        state.cursor_position.set(0);
        state
            .cursor_char
            .set(input.chars().nth(0).unwrap_or(' ').to_string());
        state.cursor_prefix.set("".to_string());
    }
}
