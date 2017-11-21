use glib;

error_chain!{
    foreign_links {
        GlibBoolError(glib::BoolError);
        // None(::std::option::NoneError);
        // Diesel(DieselError);
        // Io(::std::io::Error) #[cfg(unix)];
    }

    errors {
        WidgetNotFound(name: &'static str) {
            description("invalid toolchain name")
            display("widget wasn't found in glade file: '{}'", name)
        }

        // You can also add commas after description/display.
        // This may work better with some editor auto-indentation modes:
        DowncastFailed(from: &'static str, to: &'static str) {
            description("unknown toolchain version"), // note the ,
            display("Downcast from '{}' to '{}' failed", from, to), // trailing comma is allowed
        }
    }
}
