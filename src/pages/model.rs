use crate::ml;
use crate::utils;
use gtk::prelude::*;
use ndarray::prelude::*;
use plotters::prelude::*;
use polars::prelude::DataFrame;

use std::cell::RefCell;
use std::rc::Rc;

pub fn render_page(window: &gtk::ApplicationWindow, df_cell: Rc<RefCell<Option<DataFrame>>>) {
    let (train_set, test_set) = ml::split(df_cell.borrow().as_ref().unwrap(), 0.7);
    let weights: Rc<RefCell<Option<Array2<f64>>>> = Rc::new(RefCell::new(None));
    let bias: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(None));

    let vbox = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Vertical)
        .margin(10)
        .spacing(10)
        .build();

    // Params Inputs

    let params_box = gtk::GridBuilder::new()
        .row_spacing(10)
        .column_spacing(10)
        .hexpand(true)
        .build();
    vbox.pack_start(&params_box, false, false, 0);

    params_box.attach(
        &gtk::LabelBuilder::new().label("Learning Rate").build(),
        0,
        0,
        1,
        1,
    );
    let lr_text = gtk::TextViewBuilder::new()
        .buffer(&gtk::TextBufferBuilder::new().text("1").build())
        .hexpand(true)
        .border_width(5)
        .build();
    params_box.attach(&lr_text, 1, 0, 1, 1);

    params_box.attach(
        &gtk::LabelBuilder::new().label("Iterations").build(),
        2,
        0,
        1,
        1,
    );
    let iterations_text = gtk::TextViewBuilder::new()
        .buffer(&gtk::TextBufferBuilder::new().text("100").build())
        .hexpand(true)
        .border_width(5)
        .build();
    params_box.attach(&iterations_text, 3, 0, 1, 1);

    // Train Button

    let graph_box = gtk::BoxBuilder::new().build();
    let graph_box_clone = graph_box.clone();
    let train_button = gtk::ButtonBuilder::new().label("Train").build();
    let weights_cloned = Rc::clone(&weights);
    let bias_cloned = Rc::clone(&bias);
    train_button.connect_clicked(move |_| {
        let lr = utils::get_text(lr_text.get_buffer().unwrap())
            .parse::<f64>()
            .unwrap();
        let iterations = utils::get_text(iterations_text.get_buffer().unwrap())
            .parse::<usize>()
            .unwrap();

        let (costs, trained_weights, trained_bias) = ml::train(&train_set, lr, iterations);
        RefCell::replace(&weights_cloned, Some(trained_weights));
        RefCell::replace(&bias_cloned, Some(trained_bias));

        draw_costs_graph(&graph_box_clone, costs, iterations);
    });
    vbox.pack_start(&train_button, false, false, 0);
    vbox.pack_start(&graph_box, true, true, 0);

    // Test Button

    let test_button = gtk::ButtonBuilder::new().label("Test").build();
    vbox.pack_start(&test_button, false, false, 0);

    let diff_window = gtk::ScrolledWindowBuilder::new()
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .build();
    vbox.pack_start(&diff_window, true, true, 0);

    let accuracy_box = gtk::GridBuilder::new()
        .row_spacing(10)
        .column_spacing(10)
        .hexpand(true)
        .build();
    vbox.pack_start(&accuracy_box, false, false, 0);

    accuracy_box.attach(
        &gtk::LabelBuilder::new().label("Accuracy").build(),
        0,
        0,
        1,
        1,
    );
    let accuracy_text = gtk::TextViewBuilder::new()
        .buffer(&gtk::TextBufferBuilder::new().text("-").build())
        .border_width(5)
        .editable(false)
        .build();
    accuracy_box.attach(&accuracy_text, 1, 0, 1, 1);

    // TODO precision, recall and f1

    let weights_cloned = weights.clone();
    let bias_cloned = bias.clone();
    test_button.connect_clicked(move |_| {
        let trained_weights = weights_cloned.borrow();
        let trained_weights = trained_weights.as_ref().unwrap();
        let trained_bias = bias_cloned.borrow();
        let trained_bias = trained_bias.as_ref().unwrap();

        let (df, acc) = ml::make_prediction(&test_set, &trained_weights, &trained_bias);

        let tree_view = utils::create_tree_view(&df);
        tree_view.show();

        diff_window.add(&tree_view);

        accuracy_text
            .get_buffer()
            .unwrap()
            .set_text(&format!("{:.3}", acc));
    });

    // Window

    window.add(&utils::wrap_in_header(
        "Model",
        "Train and test a linear regression model on the data",
        &vbox,
    ));
    window.show_all();
}

fn draw_costs_graph(container: &gtk::Box, costs: Vec<f64>, iterations: usize) {
    utils::kill_children(container);

    let low = costs
        .iter()
        .cloned()
        .min_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN"))
        .unwrap()
        - 0.2;
    let high = costs
        .iter()
        .cloned()
        .min_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN"))
        .unwrap()
        + 0.2;

    let drawing_area = gtk::DrawingArea::new();
    container.pack_start(&drawing_area, true, true, 0);

    drawing_area.connect_draw(move |da, cr| {
        let root_area = plotters_cairo::CairoBackend::new(
            cr,
            (
                da.get_allocated_width() as u32,
                da.get_allocated_height() as u32,
            ),
        )
        .unwrap()
        .into_drawing_area();

        root_area.fill(&WHITE).unwrap();

        let mut ctx = ChartBuilder::on(&root_area)
            .margin(20)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Right, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption("Cost vs Iterations", ("sans-serif", 15))
            .build_cartesian_2d(0..iterations, low..high)
            .unwrap();

        ctx.configure_mesh().draw().unwrap();

        let graph = costs
            .clone()
            .into_iter()
            .enumerate()
            .map(|(idx, val)| (idx + 1, val));

        ctx.draw_series(LineSeries::new(graph, &RED)).unwrap();

        gtk::Inhibit(false)
    });

    container.show_all();
}
