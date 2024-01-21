use leptos::*;
use charming::{
    component::{Axis, DataZoom, Grid, Legend, Title},
    datatype::CompositeValue,
    element::{
        AreaStyle, AxisPointer, AxisPointerType, AxisType, DataBackground, LineStyle, SplitLine,
        TextStyle, Tooltip, Trigger, Emphasis, Label, LabelLine, LabelPosition,
    },
    series::{Candlestick, Line, Bar, Pie},
    Chart,
    WasmRenderer,
    Echarts,
};
use leptos::html::Div;
use leptos::logging::log;
use web_sys::Element;


#[component]
pub fn GraphExample() -> impl IntoView {

    let graph_ref = create_node_ref::<Div>();

    let (chart_func, set_chart_func) = create_signal("BasicLineChart".to_string());
    let rend: RwSignal<WasmRenderer> = create_rw_signal(WasmRenderer::new(1000, 800)); 
    let cheert: RwSignal<Option<Echarts>> = create_rw_signal(None); 

    let render_resource = create_local_resource(|| (), move |_| async move {
        // let chart = match chart_func.get().as_str() {
        //     "BasicLineChart" => basic_line_chart(),
        //     "BasicBarChart" => basic_bar_chart(),
        //     "PieChart" => pie_chart(),
        //     _ => basic_line_chart(),
        // };

        rend.with(|r| 
            cheert.update(|ec| *ec = Some(r.render("test_chart", &basic_line_chart()).unwrap()))
            );
        });

    let change_chart = move |ev| rend.with(|r| {
        cheert.with(|c| WasmRenderer::update(c.as_ref().unwrap(), &get_chart(ev)))
    });


    // Render the chart in the WebAssembly runtime

    view! {
        // <div class="justify-center text-center justify-items-center mx-auto">
        <div class="container w-fit mx-auto" id="test_chart" _ref=graph_ref></div>
        <select
            on:change=move |ev| change_chart(event_target_value(&ev))>
            <option value="BasicLineChart">"Line Chart"</option> 
            <option value="BasicBarChart">"Bar Chart"</option>
            <option value="PieChart">"Pie Chart"</option>
        </select> 
        // </div>
    }
}

fn get_chart(chart_name: String) -> Chart {
    match chart_name.as_str() {
        "BasicLineChart" => basic_line_chart(),
        "BasicBarChart" => basic_bar_chart(),
        "PieChart" => pie_chart(),
        _ => basic_line_chart(),
    }
}


fn basic_line_chart() -> Chart {

    Chart::new()
        .title(Title::new().text("Line Chart"))
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
        .title(Title::new().text("Bar Chart"))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Bar::new().data(vec![120, 200, 150, 80, 70, 110, 130]))
}


pub fn pie_chart() -> Chart {
    Chart::new()
        .tooltip(Tooltip::new().trigger(Trigger::Item))
        .legend(Legend::new().top("5%").left("center"))
        .series(
            Pie::new()
                .name("Access From")
                .radius(vec!["40%", "55%"])
                .avoid_label_overlap(false)
                .label(Label::new().show(false).position(LabelPosition::Center))
                .emphasis(
                    Emphasis::new()
                        .label(Label::new().show(true).font_size(40).font_weight("bold")),
                )
                .label_line(LabelLine::new().show(false))
                .data(vec![
                    (1048, "Search Engine"),
                    (735, "Direct Access"),
                    (580, "Email Marketing"),
                    (484, "Union Ads"),
                    (300, "Video Ads"),
                ]),
        )
}