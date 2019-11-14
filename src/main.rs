mod collidable;
mod obj;
mod render;
mod chunk;
mod cam;
mod ray;

use render::{Renderer, RenderSettings};
use std::time::{Duration, Instant};
use minifb::{Window, WindowOptions, Key};
use simplelog::*;
use log::{info, debug, trace};

const CHUNK_SIZE : usize = 128;
const WIDTH : usize = 1920;
const HEIGHT : usize = 1080;


fn main() {

    let rset = RenderSettings {
        chunk_size: CHUNK_SIZE,
        width: WIDTH,
        height: HEIGHT
    };

    SimpleLogger::init(LevelFilter::Trace, ConfigBuilder::new().set_time_format_str("%H:%M:%S%.6f").build()).unwrap();
    info!("Starting RustRay");
    //start measure
    
    
    
    let mut window = Window::new("RustRay", rset.width, rset.height, WindowOptions::default()).expect("Failed to create window");
    debug!("Created Window");
    //Create objects



    let mut buffer = vec![0u32; rset.width * rset.height];
    debug!("Created 0ed buffer");

    let cam = cam::Camera::new((10.0, 0.0, 2.0).into(), (-0.1, 0.0, 0.0).into(), (0.1 * 16.0 /9.0, 0.1), (rset.width, rset.height));

    let mut renderer = Renderer::new(rset.width, rset.height, rset.chunk_size);
    renderer.start_render(4, rset, cam);

    let time_render = Instant::now();
    let mut last_draw = Instant::now();
    let mut chunks_since_draw = 0;
    while let Ok(chnk) = renderer.get_next_finished() {
        trace!("Received chunk from renderer, writing to buffer and updating screen");
        chnk.copy_to_base(WIDTH, &mut buffer);
        chunks_since_draw += 1;
        if last_draw.elapsed().as_millis() > 30 {
            window.update_with_buffer(&buffer).expect("update_with_buffer failed");
            debug!("Updating screen with {} new chunks drawn", chunks_since_draw);
            chunks_since_draw = 0;
            last_draw = Instant::now();
        }
    }
    if chunks_since_draw > 0 {
        debug!("Updating screen with {} new chunks drawn", chunks_since_draw);
        window.update_with_buffer(&buffer).expect("update_with_buffer failed");
    }
    info!("Time for render and displaying: {}s", time_render.elapsed().as_secs_f64());
    //stop time

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();
        std::thread::sleep(Duration::from_millis(30));
    }
}



