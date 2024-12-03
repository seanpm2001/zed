use std::ops::Range;

use gpui::{AppContext, Model, ModelContext};
use language::Buffer;
use text::Rope;

// TODO: Find a better home for `Direction`.
//
// This should live in an ancestor crate of `editor` and `inline_completion`,
// but at time of writing there isn't an obvious spot.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Prev,
    Next,
}

#[derive(Clone)]
pub struct CompletionProposal {
    pub edits: Vec<CompletionEdit>,
}

#[derive(Clone)]
pub struct CompletionEdit {
    pub text: Rope,
    pub range: Range<language::Anchor>,
}

pub trait InlineCompletionProvider: 'static + Sized {
    fn name() -> &'static str;
    fn is_enabled(
        &self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &AppContext,
    ) -> bool;
    fn refresh(
        &mut self,
        buffer: Model<Buffer>,
        cursor_position: language::Anchor,
        debounce: bool,
        cx: &mut ModelContext<Self>,
    );
    fn cycle(
        &mut self,
        buffer: Model<Buffer>,
        cursor_position: language::Anchor,
        direction: Direction,
        cx: &mut ModelContext<Self>,
    );
    fn accept(&mut self, cx: &mut ModelContext<Self>);
    fn discard(&mut self, should_report_inline_completion_event: bool, cx: &mut ModelContext<Self>);
    fn active_completion_text<'a>(
        &'a self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &'a AppContext,
    ) -> Option<CompletionProposal>;
}

pub trait InlineCompletionProviderHandle {
    fn is_enabled(
        &self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &AppContext,
    ) -> bool;
    fn refresh(
        &self,
        buffer: Model<Buffer>,
        cursor_position: language::Anchor,
        debounce: bool,
        cx: &mut AppContext,
    );
    fn cycle(
        &self,
        buffer: Model<Buffer>,
        cursor_position: language::Anchor,
        direction: Direction,
        cx: &mut AppContext,
    );
    fn accept(&self, cx: &mut AppContext);
    fn discard(&self, should_report_inline_completion_event: bool, cx: &mut AppContext);
    fn active_completion_text<'a>(
        &'a self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &'a AppContext,
    ) -> Option<CompletionProposal>;
}

impl<T> InlineCompletionProviderHandle for Model<T>
where
    T: InlineCompletionProvider,
{
    fn is_enabled(
        &self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &AppContext,
    ) -> bool {
        self.read(cx).is_enabled(buffer, cursor_position, cx)
    }

    fn refresh(
        &self,
        buffer: Model<Buffer>,
        cursor_position: language::Anchor,
        debounce: bool,
        cx: &mut AppContext,
    ) {
        self.update(cx, |this, cx| {
            this.refresh(buffer, cursor_position, debounce, cx)
        })
    }

    fn cycle(
        &self,
        buffer: Model<Buffer>,
        cursor_position: language::Anchor,
        direction: Direction,
        cx: &mut AppContext,
    ) {
        self.update(cx, |this, cx| {
            this.cycle(buffer, cursor_position, direction, cx)
        })
    }

    fn accept(&self, cx: &mut AppContext) {
        self.update(cx, |this, cx| this.accept(cx))
    }

    fn discard(&self, should_report_inline_completion_event: bool, cx: &mut AppContext) {
        self.update(cx, |this, cx| {
            this.discard(should_report_inline_completion_event, cx)
        })
    }

    fn active_completion_text<'a>(
        &'a self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &'a AppContext,
    ) -> Option<CompletionProposal> {
        self.read(cx)
            .active_completion_text(buffer, cursor_position, cx)
    }
}
