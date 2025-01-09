use fltk::{app, button::Button, frame::Frame, input::Input, prelude::*, window::Window, group::Flex};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Form Example");

    let mut flex = Flex::default().size_of_parent().column();

    let label1 = Frame::default().with_label("Label 1:");
    let input1 = Input::default();
    flex.fixed(&label1, 30);
    flex.fixed(&input1, 30);

    let label2 = Frame::default().with_label("Label 2:");
    let input2 = Input::default();
    flex.fixed(&label2, 30);
    flex.fixed(&input2, 30);

    let label3 = Frame::default().with_label("Label 3:");
    let input3 = Input::default();
    flex.fixed(&label3, 30);
    flex.fixed(&input3, 30);

    let label4 = Frame::default().with_label("Label 4:");
    let input4 = Input::default();
    flex.fixed(&label4, 30);
    flex.fixed(&input4, 30);

    let mut ok_button = Button::default().with_label("OK");
    ok_button.set_callback(|_| {
        println!("Button clicked!");
    });
    flex.fixed(&ok_button, 30);

    flex.end();
    wind.end();
    wind.show();

    app.run().unwrap();
}