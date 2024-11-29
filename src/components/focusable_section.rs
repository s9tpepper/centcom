use anathema::{
    component::Component,
    state::{State, Value},
};

#[derive(Default)]
pub struct FocusableSection;

#[derive(Default, State)]
pub struct FocusableSectionState {
    target: Value<Option<String>>,
    active_border_color: Value<String>,
}

impl FocusableSectionState {
    pub fn new() -> Self {
        FocusableSectionState {
            target: None.into(),
            active_border_color: String::from("#666666").into(),
        }
    }
}

impl Component for FocusableSection {
    type State = FocusableSectionState;
    type Message = ();

    fn accept_focus(&self) -> bool {
        false
    }

    fn tick(
        &mut self,
        state: &mut Self::State,
        _elements: anathema::widgets::Elements<'_, '_>,
        context: anathema::prelude::Context<'_, Self::State>,
        _dt: std::time::Duration,
    ) {
        if state.target.to_ref().is_some() {
            return;
        }

        let Some(target) = context.get_external("target") else {
            return;
        };

        if let Some(target) = target.to_common() {
            state.target.set(Some(target.to_string()));
        }
    }

    fn receive(
        &mut self,
        ident: &str,
        value: anathema::state::CommonVal<'_>,
        state: &mut Self::State,
        _: anathema::widgets::Elements<'_, '_>,
        _: anathema::prelude::Context<'_, Self::State>,
    ) {
        if state.target.to_ref().is_none() {
            return;
        }

        #[allow(clippy::single_match)]
        match ident {
            "input_focus" => {
                let focus = value.to_bool();
                match focus {
                    true => state.active_border_color.set("#ffff00".to_string()),
                    false => state.active_border_color.set("#666666".to_string()),
                }
            }

            _ => {}
        }
    }
}
