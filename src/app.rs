#[cfg(target_os = "windows")]

use winapi::shared::windef::{HWND, RECT};
use winapi::um::winuser::GetClientRect;

use direct2d::render_target::hwnd::builder::HwndRenderTargetBuilder;
use direct2d::render_target::HwndRenderTarget;
use direct2d::brush::SolidColorBrush;
use direct2d::Factory;

use rand::prelude::*;

// container for useful use the app data
pub struct AppModel {
    pub colors: Vec<u32>,
}

impl AppModel {
    pub fn new(colors: Vec<u32>) -> AppModel {
        AppModel { colors }
    }

    pub fn new_random(length: usize) -> AppModel {
        let mut result = AppModel::new(vec![0; length]);
        result.random();
        result
    }

    pub fn random(&mut self) {
        let mut rnd = rand::thread_rng();

        for color in self.colors.iter_mut() {
            *color = rnd.gen_range(0, 0xFF_FF_FF) as u32;
        }
    }
}

pub struct App {
    render: HwndRenderTarget,
    model: AppModel,

    #[allow(dead_code)]
    hwnd: HWND,
}

impl App {
    pub fn with_size(hwnd: HWND, width: u32, height: u32) -> App {
        // Make new factory just only for create render target
        let factory = Factory::new().unwrap();

        // Initialize render target by hwnd of the window
        // and set the canvas size.
        // also, you can create render target by HDC.
        let target = HwndRenderTargetBuilder::new(&factory)
            .with_pixel_size(width, height)
            .with_hwnd(hwnd)
            .build().unwrap();

        App {
            render: target,
            model: AppModel::new_random(4),
            hwnd,
        }
    }

    pub fn new(hwnd: HWND) -> App {
        let (width, height) = get_client_size(hwnd);
        App::with_size(hwnd, width, height)
    }

    pub fn draw(&mut self) {
        let size = self.render.pixel_size();
        let block_width = (size.width / (self.model.colors.len() as u32)) as f32;
        let mut last_position = 0.0;

        self.render.begin_draw();

        for color in self.model.colors.iter() {
            // because crate direct2d is not
            // implements change color of the
            // brush currently, we should creates
            // new brush in each iteration.
            let temp_brush = SolidColorBrush::create(&self.render)
                .with_color(*color as u32)
                .build().unwrap();

            self.render.fill_rectangle([
                last_position, 0.0, last_position + block_width + 1.0, size.height as f32
            ], &temp_brush);

            last_position += block_width;
        }

        self.render.end_draw().unwrap();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.render.resize((width, height).into()).unwrap();
    }

    pub fn random(&mut self) {
        self.model.random();
    }
}

// get window client size
fn get_client_size(hwnd: HWND) -> (u32, u32) {
    unsafe {
        let mut buffer : RECT = std::mem::uninitialized();
        GetClientRect(hwnd, &mut buffer);

        (buffer.right as u32, buffer.bottom as u32)
    }
}
