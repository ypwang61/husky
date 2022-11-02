mod figure_canvas;
mod figure_control;

use super::*;
use figure_canvas::*;
use figure_control::*;

#[derive(Prop, Clone)]
pub struct FigureViewProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
}

const FIGURE_TOP_BAR_HEIGHT: u32 = 23;

impl<'a> FigureViewProps<'a> {
    fn content_dimension(&self) -> PixelDimension {
        let dimension = self.dimension.cget();
        PixelDimension {
            width: dimension.width - 4,
            height: dimension.height - FIGURE_TOP_BAR_HEIGHT,
        }
    }
    fn title_dimension(&self) -> PixelDimension {
        let dimension = self.dimension.cget();
        PixelDimension {
            width: dimension.width,
            height: FIGURE_TOP_BAR_HEIGHT,
        }
    }
}

#[component]
pub fn FigureView<'a, G: Html>(scope: Scope<'a>, props: FigureViewProps<'a>) -> View<G> {
    let dimension = props.dimension;
    let content_dimension = memo!(scope, move || props.content_dimension(), props);
    let title_dimension = memo!(scope, move || props.title_dimension(), props);
    let ctx = use_dev_context(scope);
    let figure_canvas_value_signal = memo!(scope, move || {
        let presentation = &ctx.presentation_signal().get();
        let opt_active_figure_not_pinned = presentation
            .opt_active_trace_id()
            .map(|trace_id| {
                if presentation.is_pinned(trace_id) {
                    None
                } else {
                    Some(ctx.figure_canvas_data_itd(trace_id, presentation))
                }
            })
            .flatten();
        let value = FigureCanvasValue::new(
            presentation.kind(),
            opt_active_figure_not_pinned,
            ctx.figure_canvas_data_itds(presentation),
        );
        value
    });
    let presentation_kind = memo!(scope, move || ctx.presentation_signal().get().kind());
    view! {
        scope,
        div (class="FigureView disable-select") {
            div (
                class="FigureTitle",
                style=title_dimension.cget().to_style(),
            ) {
                label { "title" }
            }
            div (
                class="FigureCanvasWrapper",
                style=dimension.cget().to_style(),
            ) {
                FigureCanvas {
                    dimension: content_dimension,
                    value: figure_canvas_value_signal,
                    presentation_kind,
                }
            }
        }
    }
}
