extern crate gtk;

use gtk::prelude::*;

use gtk::{
    Button,
    Orientation,
    Stack,
    StackSwitcher,
    StackTransitionType,
    TextView,
    Window,
    WindowType,
};


fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Hello GTK+");
    window.set_default_size(400, 100);

    let window_box = gtk::Box::new(Orientation::Vertical, 6);

    let stack = Stack::new();
    stack.set_transition_type(StackTransitionType::SlideLeftRight);
    stack.set_transition_duration(1000);

    let button = Button::new_with_label("Click me!");
    let text_view = TextView::new();

    stack.add_titled(&button, "button_example", "Button Example!");
    stack.add_titled(&text_view, "text_view", "Text View!");

    let stack_switcher = StackSwitcher::new();

    stack_switcher.set_stack(&stack);

    window_box.pack_start(&stack_switcher, true, true, 0);
    window_box.pack_start(&stack, true, true, 0);

    window.add(&window_box);
    window.show_all();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        println!("Clicked!");
    });

    gtk::main();
}
