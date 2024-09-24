use std::ops::ControlFlow;

use anathema::geometry::{LocalPos, Size};
use anathema::state::CommonVal;
use anathema::widgets::layout::text::{ProcessResult, Segment, Strings};
use anathema::widgets::layout::{Constraints, LayoutCtx, PositionCtx};
use anathema::widgets::paint::{PaintCtx, SizePos};
use anathema::widgets::{
    AttributeStorage, LayoutChildren, PaintChildren, PositionChildren, Widget, WidgetId,
};

const LEFT: &str = "left";
const RIGHT: &str = "right";

pub(crate) const WRAP: &str = "wrap";
pub(crate) const TEXT_ALIGN: &str = "text_align";

/// Text alignment aligns the text inside its parent.
///
/// Given a border with a width of nine and text alignment set to [`TextAlignment::Right`]:
/// ```text
/// ┌───────┐
/// │I would│
/// │ like a│
/// │ lovely│
/// │ cup of│
/// │    tea│
/// │ please│
/// └───────┘
/// ```
///
/// The text will only align it self within the parent widget.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub enum TextAlignment {
    /// Align the to the left inside the parent
    #[default]
    Left,
    /// Align the text in the centre of the parent
    Centre,
    /// Align the to the right inside the parent
    Right,
}

impl TryFrom<CommonVal<'_>> for TextAlignment {
    type Error = ();

    fn try_from(value: CommonVal<'_>) -> Result<Self, Self::Error> {
        match value {
            CommonVal::Str(wrap) => match wrap {
                LEFT => Ok(TextAlignment::Left),
                RIGHT => Ok(TextAlignment::Right),
                "centre" | "center" => Ok(TextAlignment::Centre),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}

/// Text widget
/// ```ignore
/// Attributes:
/// * background
/// * foreground
/// * text-align
/// * wrap
/// ```
///
/// Note: Spans, unlike other widgets, does not require a widget id
///
/// A `Text` widget will be as wide as its text.
#[derive(Debug, Default)]
pub struct Text {
    strings: Strings,
}

impl Widget for Text {
    fn layout<'bp>(
        &mut self,
        mut children: LayoutChildren<'_, '_, 'bp>,
        constraints: Constraints,
        id: WidgetId,
        ctx: &mut LayoutCtx<'_, 'bp>,
    ) -> Size {
        let attributes = ctx.attribs.get(id);
        let wrap = attributes.get(WRAP).unwrap_or_default();
        let size = constraints.max_size();
        self.strings = Strings::new(size, wrap);
        self.strings.set_style(id);

        // Layout text
        attributes.value().map(|value| {
            value.str_iter(|s| match self.strings.add_str(s) {
                ProcessResult::Break => ControlFlow::Break(()),
                ProcessResult::Continue => ControlFlow::Continue(()),
            })
        });

        // Layout text of all the sub-nodes
        children.for_each(|child, _| {
            let Some(_span) = child.try_to_ref::<Span>() else {
                return ControlFlow::Continue(());
            };
            self.strings.set_style(child.id());

            let attributes = ctx.attribs.get(child.id());
            if let Some(text) = attributes.value() {
                text.str_iter(|s| match self.strings.add_str(s) {
                    ProcessResult::Break => ControlFlow::Break(()),
                    ProcessResult::Continue => ControlFlow::Continue(()),
                })?;

                ControlFlow::Continue(())
            } else {
                ControlFlow::Break(())
            }
        });

        self.strings.finish()
    }

    fn paint<'bp>(
        &mut self,
        _: PaintChildren<'_, '_, 'bp>,
        id: WidgetId,
        attribute_storage: &AttributeStorage<'bp>,
        mut ctx: PaintCtx<'_, SizePos>,
    ) {
        let lines = self.strings.lines();
        let alignment = attribute_storage
            .get(id)
            .get(TEXT_ALIGN)
            .unwrap_or_default();

        let mut pos = LocalPos::ZERO;
        let mut style = attribute_storage.get(id);

        for line in lines {
            let x = match alignment {
                TextAlignment::Left => 0,
                TextAlignment::Centre => ctx.local_size.width as u16 / 2 - line.width / 2,
                TextAlignment::Right => ctx.local_size.width as u16 - line.width,
            };

            pos.x = x;

            for entry in line.entries {
                match entry {
                    Segment::Str(s) => {
                        if let Some(new_pos) = ctx.place_glyphs(s, pos) {
                            // NOTE:
                            // This isn't very nice, but it works for now.
                            // In the future there should probably be a means to
                            // provide both style and glyph at the same time.
                            for x in pos.x..new_pos.x {
                                ctx.set_attributes(style, (x, pos.y).into());
                            }
                            pos = new_pos;
                        }
                    }
                    Segment::SetStyle(attribute_id) => style = attribute_storage.get(attribute_id),
                }
            }
            pos.y += 1;
            pos.x = 0;
        }
    }

    fn position<'bp>(
        &mut self,
        _children: PositionChildren<'_, '_, 'bp>,
        _attributes: WidgetId,
        _attribute_storage: &AttributeStorage<'bp>,
        _ctx: PositionCtx,
    ) {
        // NOTE
        // No positioning is done in here, it's all done when painting
    }
}

#[derive(Default, Copy, Clone)]
pub struct Span;

impl Widget for Span {
    fn layout<'bp>(
        &mut self,
        _: LayoutChildren<'_, '_, 'bp>,
        _: Constraints,
        _: WidgetId,
        _: &mut LayoutCtx<'_, 'bp>,
    ) -> Size {
        // Everything is handled by the parent text
        panic!("this should never be called");
    }

    fn position<'bp>(
        &mut self,
        _: PositionChildren<'_, '_, 'bp>,
        _: WidgetId,
        _: &AttributeStorage<'bp>,
        _: PositionCtx,
    ) {
        // Everything is handled by the parent text
        panic!("this should never be called");
    }
}