use orbtk::{Button, Click, Place, Point, Rect, Text, Window};
use std::fs::{File, read_to_string};
use std::io::Read;
use std::process::exit;

fn check_x(x_value: i32) -> bool {
    if x_value > 1100 {
        return true;
    } else {
        return false;
    }
}

pub fn installed_init() {
    let mut window = Window::new(Rect::new(100, 100, 1100, 600),"Smoke Installer - Store");

    let installed = read_to_string("/etc/smoke_installer/installed.txt").unwrap();

    let mut x = 0;
    let mut y = 20;

    // Buttons
    let open_hello_smoke = hello_smoke::installed::open();
    // -------

    // Check if it's installed
    if installed.contains("hello-smoke-installer") {
        open_hello_smoke.position(x, y);
        window.add(&open_hello_smoke);
        if check_x(x) {
            x = x + 75;
        } else {
            y = y + 75;
            x = 0;
        }
    }
    // ---

    let exit_btn = Button::new();
    exit_btn
        .position( 1055, 0).text("Exit")
        .on_click(move |_exit_btn: &Button, _point: Point| {
            exit(0);
        });

    window.add(&exit_btn);
    window.exec();
}
