use gio::prelude::*;
use gtk::prelude::*;
use polars::prelude::*;

pub fn wrap_in_header(title: &str, subtitle: &str, content: &gtk::Box) -> gtk::Box {
    let vbox = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let header = gtk::HeaderBarBuilder::new()
        .title(title)
        .subtitle(subtitle)
        .build();

    vbox.pack_start(&header, false, false, 0);
    vbox.pack_start(content, true, true, 0);

    vbox
}

pub fn create_tree_view(dataframe: &polars::frame::DataFrame) -> gtk::TreeView {
    let dtypes = vec![f64::static_type(); dataframe.width()];
    let store = gtk::TreeStore::new(&dtypes);
    let columns: Vec<u32> = (0..dataframe.width()).map(|x| x as u32).collect();

    for idx in 0..dataframe.height() {
        let row_vals: Vec<f64> = dataframe
            .get_row(idx)
            .0
            .into_iter()
            .map(|x| match x {
                AnyValue::Int64(x) => x as f64,
                AnyValue::Float64(x) => x,
                _ => panic!("couldn't cast the dataframe datatype"),
            })
            .collect();

        let mut row: Vec<&dyn ToValue> = Vec::new();
        for cell in row_vals.iter() {
            row.push(cell);
        }

        store.set(&store.append(None), &columns, &row);
    }

    let tree_view = gtk::TreeViewBuilder::new()
        .enable_grid_lines(gtk::TreeViewGridLines::Both)
        .model(&store)
        .build();

    let renderer = gtk::CellRendererTextBuilder::new()
        .foreground_rgba(&gdk::RGBA {
            red: 0.8,
            blue: 0.8,
            green: 0.8,
            alpha: 1.0,
        })
        .xalign(1.0)
        .build();

    for (idx, header) in dataframe.get_column_names().into_iter().enumerate() {
        let column = gtk::TreeViewColumnBuilder::new()
            .title(header)
            .sizing(gtk::TreeViewColumnSizing::Autosize)
            .sort_column_id(idx as i32)
            .build();
        column.pack_start(&renderer, true);
        column.add_attribute(&renderer, "text", idx as i32);

        tree_view.append_column(&column);
    }

    tree_view
}

pub fn kill_children<T: IsA<gtk::Container>>(widget: &T) {
    let children = widget.get_children();
    unsafe {
        children.iter().for_each(|x| x.destroy());
    }
}

pub fn get_text(buffer: gtk::TextBuffer) -> String {
    buffer
        .get_text(&buffer.get_start_iter(), &buffer.get_end_iter(), false)
        .unwrap()
        .to_string()
}
