///
/// Most of this is just a copy of the examples in the repo
extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use gtk::prelude::*;

use gtk::{
    ContainerExt,
    BoxExt,
    Button,
    ButtonExt,
    Inhibit,
    Label,
    LabelExt,
    Stack,
    StackSwitcher,
    StackTransitionType,
    TextView,
    TextViewExt,
    TextBuffer,
    TextBufferExt,
    WidgetExt,
    Window,
    WindowType,
};
use gtk::Orientation::Vertical;
use relm::{Relm, Update, Widget};

#[derive(Msg)]
pub enum Msg {
    Change,
    Clicked,
    Quit,
}

pub struct Model {
    content: String,
    clicks: i32,
}

impl Model {
    fn format_content(&self) -> String {
        format!("Clicked {} times! Input Text:\n{}", self.clicks, self.content)
    }
}

pub struct Win {
    button: Button,
    label: Label,
    text_buffer: TextBuffer,
    model: Model,
    window: Window,
}

impl Update for Win {

    // Model used for the widget
    type Model = Model;

    // Model parameter used to initialize model
    type ModelParam = ();

    type Msg = Msg;

    // Return the initial model
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            content: String::new(),
            clicks: 0,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Change => {
                let text = self.text_buffer.get_text(
                    &self.text_buffer.get_start_iter(),
                    &self.text_buffer.get_end_iter(),
                    false, // don't show invisible text
                ).unwrap();
                self.model.content = text;
                self.label.set_text(&self.model.format_content());
            },
            Msg::Clicked => {
                self.model.clicks += 1;
                self.button.set_label(
                    &format!("Clicks: {}", self.model.clicks)
                );
                self.label.set_text(&self.model.format_content());
            }
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {

    // The root widget is a gtk window
    type Root = Window;

    // Returns the root widget
    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let vbox = gtk::Box::new(Vertical, 0);

        let stack = Stack::new();
        stack.set_transition_type(StackTransitionType::SlideLeftRight);
        stack.set_transition_duration(1000);

        let button = Button::new_with_label("Clicks: 0");
        let label = Label::new(None);
        let text_view = TextView::new();

        stack.add_titled(&button, "button_example", "Button Example!");
        stack.add_titled(&text_view, "text_view", "Text View!");
        stack.add_titled(&label, "info", "Info!");
        let stack_switcher = StackSwitcher::new();
        stack_switcher.set_stack(&stack);

        vbox.pack_start(&stack_switcher, true, true, 0);
        vbox.pack_start(&stack, true, true, 0);

        let window = Window::new(WindowType::Toplevel);
        window.set_title("Hello relm!");

        window.add(&vbox);

        window.show_all();

        let text_buffer = text_view.get_buffer().unwrap();

        connect!(
            relm,
            text_buffer,
            connect_changed(_),
            Msg::Change
        );

        connect!(
            relm,
            button,
            connect_clicked(_),
            Msg::Clicked
        );

        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        Win {
            button,
            label,
            text_buffer,
            model,
            window,
        }
    }

}


fn main() {
    Win::run(()).unwrap();
}
