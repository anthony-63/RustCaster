mod loader;
mod renderer;

use macroquad::prelude::*;
use renderer::tdim::TDimRenderer;

fn window_conf() -> Conf {
    Conf {
        window_title: "RustCaster | 9999.99FPS".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut tdim_renderer = TDimRenderer::new();

    loop {
        clear_background(BLACK);
        tdim_renderer.render();
        tdim_renderer.update();
        next_frame().await
    }
}
