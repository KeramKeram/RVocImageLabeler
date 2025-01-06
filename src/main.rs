use fltk::{app, button::Button, frame::Frame, input::Input, prelude::*, window::Window, group::Flex};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Form Example");

    let mut flex = Flex::default().size_of_parent().column();

    let mut label1 = Frame::default().with_label("Label 1:");
    let mut input1 = Input::default();
    flex.set_size(&label1, 30);
    flex.set_size(&input1, 30);

    let mut label2 = Frame::default().with_label("Label 2:");
    let mut input2 = Input::default();
    flex.set_size(&label2, 30);
    flex.set_size(&input2, 30);

    let mut label3 = Frame::default().with_label("Label 3:");
    let mut input3 = Input::default();
    flex.set_size(&label3, 30);
    flex.set_size(&input3, 30);

    let mut label4 = Frame::default().with_label("Label 4:");
    let mut input4 = Input::default();
    flex.set_size(&label4, 30);
    flex.set_size(&input4, 30);

    let mut ok_button = Button::default().with_label("OK");
    flex.set_size(&ok_button, 30);

    flex.end();
    wind.end();
    wind.show();

    app.run().unwrap();
}