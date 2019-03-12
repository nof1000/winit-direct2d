#![windows_subsystem = "windows"]

#[cfg(target_os = "windows")]
extern crate direct2d;
#[cfg(target_os = "windows")]
extern crate winapi;

extern crate winit;
extern crate rand;

#[cfg(target_os = "windows")]
use winit::os::windows::WindowExt;

use winit::{WindowBuilder, WindowEvent, EventsLoop, Event};
use winit::dpi::LogicalSize;

mod app;
use app::App;

#[cfg(target_os = "windows")]
fn main() {
    // creating event loop and winit window
    let mut events = EventsLoop::new();
    let window = WindowBuilder::new()
        .with_title("Winit Direct2D Example: Press \"SpaceBar\" to change color")
        .with_min_dimensions((800, 600).into())
        .build(&events).unwrap();

    // after we created the window, we can get him HWND
    // by means of winit WindowExt
    let mut app = App::new(window.get_hwnd() as _);

    events.run_forever(|event| {
        use winit::ControlFlow::{Continue, Break};

        match event {
            Event::WindowEvent{ event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == winit::ElementState::Pressed {
                        match input.virtual_keycode {
                            Some(winit::VirtualKeyCode::Space) => {
                                app.random();
                                app.draw();
                            },
                            Some(_) | None => (),
                        }
                    }
                },
                WindowEvent::Resized(LogicalSize { width, height }) => {
                    app.resize(width as _, height as _);
                },
                WindowEvent::Refresh => app.draw(),
                WindowEvent::CloseRequested => return Break,
                _ => (),
            },
            _ => (),
        }

        Continue
    });
}

#[cfg(not(target_os = "windows"))]
fn main() {
    println!("windows-only");
}
