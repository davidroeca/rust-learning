extern crate qt_core;
use qt_core::core_application::CoreApplication;

fn main() {
    CoreApplication::create_and_exit(|app| {
        CoreApplication::exec()
    })
}
