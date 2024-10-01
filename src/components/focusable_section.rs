use anathema::{
    component::Component,
    state::{CommonVal, State},
};

#[derive(Default)]
pub struct FocusableSection;

#[derive(Default, State)]
pub struct FocusableSectionState {}

impl FocusableSectionState {
    pub fn new() -> Self {
        FocusableSectionState {}
    }
}

impl Component for FocusableSection {
    type State = FocusableSectionState;
    type Message = ();

    fn accept_focus(&self) -> bool {
        false
    }

    fn receive(
        &mut self,
        ident: &str,
        value: anathema::state::CommonVal<'_>,
        _state: &mut Self::State,
        mut elements: anathema::widgets::Elements<'_, '_>,
        context: anathema::prelude::Context<'_, Self::State>,
    ) {
        let Some(target) = context.get_external("target") else {
            return;
        };
        let Some(target) = target.to_common() else {
            return;
        };

        if target.to_string() == ident {
            let focus = value.to_bool();
            if focus {
                return;
            }

            let Some(border_color) = context.get_external("border_color") else {
                return;
            };
            let Some(border_color) = border_color.to_common() else {
                return;
            };

            // NOTE: Is this right?
            let color = border_color.to_string().leak();
            elements
                .by_attribute("id", target)
                .each(|_element, attributes| {
                    attributes.set("foreground", CommonVal::Str(color));
                });
        }
    }
}
