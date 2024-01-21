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
        let chart = basic_line_chart();

                // Chart dimension 1000x800.

            let renderer = WasmRenderer::new(500, 400);
            renderer.render("test_chart", &chart).unwrap();
        });

    // Render the chart in the WebAssembly runtime

    view! {
        <div class="text-center">
            <div class="container mx-auto w-fit" id="test_chart"></div>
            <div>"Copy this repo and change the chart function in the source code and make different charts"</div>
        </div>
    }   

}

fn basic_line_chart() -> Chart {

    Chart::new()
        .title(Title::new().text("Simple Line Chart"))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]))
}

