use std::path::Path;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::env;
use echain::{ErrorKind, Result};
use gtk;
use gtk::prelude::*;
use gtk::{Builder, Window, WidgetExt, TreeView, TreeViewExt, TreeViewColumn, CellRendererText, TreeStore, TreeModel, TextTag};
use sourceview::{Buffer, BufferExt, LanguageManager, LanguageManagerExt, View};
use syntex_syntax::codemap::FilePathMapping;
use syntex_syntax::parse::{self, ParseSess};
use syntex_syntax::visit::walk_crate;

use visitor::TreeVisitor;
use tree_column_set_data_func_ext::TreeViewColumnSetCellDataFuncExt;
use ast_model_extensions::{AstModelExt, AstModelColumns, AstPropertiesColumns};

macro_rules! column {
    ($tree:expr, $col:ident, $cell:ident, $type:ident, $title:expr, $expand:expr) => {
        let $col = TreeViewColumn::new();
        $col.set_title($title);
        let $cell = $type::new();
        $col.pack_end(&$cell, $expand);
        $tree.append_column(&$col);
    };
}

fn add_ast_columns(tree: &TreeView) {
    column!(tree, type_col, type_cell, CellRendererText, "Type", true);
    type_col.add_attribute(&type_cell, "text", AstModelColumns::Type as i32);
    column!(tree, kind_col, kind_cell, CellRendererText, "Kind", true);
    kind_col.add_attribute(&kind_cell, "text", AstModelColumns::Kind as i32);

    column!(tree, span_col, span_cell, CellRendererText, "Span", false);
    span_col.set_cell_data_func(&span_cell, move |_column, cell, model, iter| {
        let text_cell = cell.clone().downcast::<CellRendererText>()
            .expect("Couldn't downcast to CellRendererText");

        if let Some((lo, hi)) = model.get_span(iter) {
            text_cell.set_property_text(Some(&format!("[{}..{})", lo, hi)));
        }
        else {
            text_cell.set_property_text(Some(""));
        }
    });
}

fn add_properties_columns(list: &TreeView) {
    column!(list, name_col, name_cell, CellRendererText, "Property", true);
    name_col.add_attribute(&name_cell, "text", AstPropertiesColumns::Name as i32);
    column!(list, value_col, value_cell, CellRendererText, "Value", true);
    value_col.add_attribute(&value_cell, "text", AstPropertiesColumns::Value as i32);
}

fn open_file<T: AsRef<Path>>(path: T, buffer: &Buffer) {
    let file = File::open(&path).expect("Couldn't open file");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Couldn't read the file");
    buffer.set_text(&contents);

    let lang_manager = LanguageManager::new();

    if let Some(lang) = lang_manager.guess_language(path.as_ref().to_str(), None) {
        buffer.set_language(Some(&lang));
    }
}


fn parse_file<T: AsRef<Path>>(path: T) -> TreeStore {
    let path_mapping = FilePathMapping::empty();
    let parse_session = ParseSess::new(path_mapping);

    let krate = match parse::parse_crate_from_file(path.as_ref(), &parse_session) {
        // There may be parse errors that the parser recovered from, which we
        // want to treat as an error.
        Ok(_) if parse_session.span_diagnostic.has_errors() => Err(None),
        Ok(krate) => Ok(krate),
        Err(e) => Err(Some(e)),
    };

    let mut vis = TreeVisitor::new();
    walk_crate(&mut vis, &krate.expect("Could not walk crate"));

    vis.tree
}

macro_rules! get_widget {
    ($builder:expr, $name:ident, $type:ty) => {
        let $name: $type = $builder.get_object(stringify!($name))
            .ok_or(ErrorKind::WidgetNotFound(stringify!($name)))?;
    }
}

pub(crate) fn gui_main() -> Result<()> {
    gtk::init()?;
    let glade_src = include_str!("syntax_visualizer.glade");
    let builder = Builder::new_from_string(glade_src);

    get_widget!(builder, main_window, Window);
    get_widget!(builder, source_view, View);
    get_widget!(builder, syntax_tree_view, TreeView);
    get_widget!(builder, node_properties_view, TreeView);

    let buffer: Buffer = source_view.get_buffer()
        .ok_or(ErrorKind::WidgetNotFound("Buffer"))?
        .downcast::<Buffer>()
        .map_err(|_| ErrorKind::DowncastFailed("TextBuffer", "Buffer"))?;

    // syntax_tree_view.set_headers_visible(false);
    add_ast_columns(&syntax_tree_view);
    add_properties_columns(&node_properties_view);

    if let Some(path) = env::args().nth(1) {
        let syntax_tree_store = parse_file(&path);
        open_file(&path, &buffer);
        syntax_tree_view.set_model(Some(&syntax_tree_store));
    }

    let tag_table = buffer.get_tag_table().ok_or(ErrorKind::WidgetNotFound("TagTable"))?;
    let tag_highlighted = TextTag::new("highlighted");
    tag_highlighted.set_property_background(Some("#dcebff"));
    tag_table.add(&tag_highlighted);

    let syntax_tree_selection = syntax_tree_view.get_selection();
    let buffer_clone = buffer.clone();
    syntax_tree_selection.connect_changed(move |tree_selection| {
        let (start_iter, end_iter) = buffer_clone.get_bounds();
        buffer_clone.remove_tag_by_name("highlighted", &start_iter, &end_iter);

        if let Some((model, iter)) = tree_selection.get_selected() {
            let props = model.get_properties_list(&iter);
            node_properties_view.set_model(Some(&props));

            if let Some((lo, hi)) = model.get_span(&iter) {
                let mut lo_iter = buffer_clone.get_iter_at_offset(lo as i32);
                let hi_iter = buffer_clone.get_iter_at_offset(hi as i32);
                buffer_clone.apply_tag_by_name("highlighted", &lo_iter, &hi_iter);
                source_view.scroll_to_iter(&mut lo_iter, 0.1, false, 0.0, 0.0);
            }
        }
        else {
            node_properties_view.set_model(None::<&TreeModel>);
        }
    });

    buffer.connect_property_cursor_position_notify(move |buffer| {
        let pos = buffer.get_property_cursor_position();
        println!("new cursor position: {}", pos);
        let model = syntax_tree_view.get_model().expect("Couldnt get tree model");
        if let Some(iter) = model.find_node_by_pos(pos) {
            let path = model.get_path(&iter).expect("Could not get tree path");
            syntax_tree_view.expand_to_path(&path);
            syntax_tree_selection.select_iter(&iter);
            syntax_tree_view.scroll_to_cell(&path, None, false, 0.0, 0.0);
        }
    });

    main_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    main_window.show_all();

    gtk::main();
    Ok(())
}
