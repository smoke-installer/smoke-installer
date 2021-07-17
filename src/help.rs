use orbtk::{Button, Click, Label, Place, Point, Rect, Text, Window};
use std::process::exit;

pub fn help_init() {
    let mut window = Window::new(Rect::new(100, 100, 1100, 600), "Smoke Installer - Help");

    let main_text = Label::new();
    main_text.position(0, 0).size(100, 100).text("Smoke Installer Help");

    let sub_text = Label::new();
    sub_text.position(0, 20).size(100, 100).text("If you have an issue with Smoke Installer, go to the help page on the smoke Installer website");

    let website_btn = Button::new();
    website_btn.position(0, 45).text("Smoke Installer Help Page").on_click(move |_website_btn: &Button, point: Point| {
        webbrowser::open("https://smoke-installer.github.io/help.html");
    });

    let exit_btn = Button::new();
    exit_btn
        .position( 1055, 0).text("Exit")
        .on_click(move |_exit_btn: &Button, _point: Point| {
            exit(0);
        });

    window.add(&main_text);
    window.add(&sub_text);
    window.add(&website_btn);
    window.add(&exit_btn);
    window.exec();
}
