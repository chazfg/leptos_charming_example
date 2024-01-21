use leptos::*;
use charming::{
    component::{Axis, DataZoom, Grid, Legend, Title},
    datatype::CompositeValue,
    element::{
        AreaStyle, AxisPointer, AxisPointerType, AxisType, DataBackground, LineStyle, SplitLine,
        TextStyle, Tooltip, Trigger,
    },
    series::{Candlestick, Line, Bar},
    Chart,
    WasmRenderer,
};


#[component]
pub fn GraphExample() -> impl IntoView {


    let render_resource = create_local_resource(|| (), |_| async move {
        let chart = basic_bar_chart();

                // Chart dimension 1000x800.

            let renderer = WasmRenderer::new(1000, 800);
            renderer.render("test_chart", &chart).unwrap();
        });

    // Render the chart in the WebAssembly runtime

    view! {
        <div class="justify-center text-center justify-items-center mx-auto">
            <div class="container w-fit" id="test_chart"></div>
        </div>
    }

}

fn basic_line_chart() -> Chart {

    Chart::new()
        .title(Title::new().text("Demo: Yew + Charming"))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]))
}


fn basic_bar_chart() -> Chart {
    Chart::new()
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Bar::new().data(vec![120, 200, 150, 80, 70, 110, 130]))
}