#[macro_use]
extern crate error_chain;
extern crate glib_sys as glib_ffi;
extern crate gtk_sys as ffi;
extern crate glib;
extern crate gtk;
extern crate sourceview;
extern crate syntex_syntax;
extern crate syntex_pos;

mod echain;
mod gui;
mod visitor;
mod tree_column_set_data_func_ext;
mod ast_model_extensions;

fn main() {
    gui::gui_main().unwrap();
}
