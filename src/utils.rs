use gio::prelude::*;
use gtk::prelude::*;
use polars::prelude::*;

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

    let tree_view = gtk::TreeView::with_model(&store);
    for (idx, header) in dataframe.get_column_names().into_iter().enumerate() {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title(header);
        column.add_attribute(&renderer, "text", idx as i32);
        column.set_sort_column_id(idx as i32);
        tree_view.append_column(&column);
    }

    tree_view
}
