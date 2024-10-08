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
    url_update: Value<String>,
    request_body_update: Value<String>,
}

impl FocusableSectionState {
    pub fn new() -> Self {
        FocusableSectionState {
            target: None.into(),
            active_border_color: String::from("#666666").into(),
            url_update: String::from("").into(),
            request_body_update: String::from("").into(),
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
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        if state.target.to_ref().is_none() {
            return;
        }

        match ident {
            "input_focus" => {
                let focus = value.to_bool();
                match focus {
                    true => state.active_border_color.set("#ffffff".to_string()),
                    false => state.active_border_color.set("#666666".to_string()),
                }
            }

            "url_update" => {
                state.url_update.set(value.to_string());

                context.publish("url_update", |state| &state.url_update)
            }

            "request_body_update" => {
                state.request_body_update.set(value.to_string());

                context.publish("request_body_update", |state| &state.request_body_update)
            }

            _ => {}
        }
    }
}
