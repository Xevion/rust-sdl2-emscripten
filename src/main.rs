use colors_transform::Color;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};
use tracing::event;
use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt;


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Setup tracing
    let subscriber = tracing_subscriber::fmt()
        .with_ansi(cfg!(not(target_os = "emscripten")))
        .with_max_level(tracing::Level::DEBUG)
        .finish()
        .with(ErrorLayer::default());

    tracing::subscriber::set_global_default(subscriber).expect("Could not set global default");

    let window = video_subsystem
        .window("Pac-Man", 500, 500)
        .position_centered()
        .build()
        .expect("Could not initialize window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not build canvas");

    canvas
        .set_logical_size(500, 500)
        .expect("Could not set logical size");

    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Could not get SDL EventPump");

    let loop_time = Duration::from_secs(1) / 60;
    let mut tick_no = 0u32;

    // The start of a period of time over which we average the frame time.
    let mut last_averaging_time = Instant::now();
    let mut sleep_time = Duration::ZERO;
    let mut paused = false;
    let mut shown = false;

    let mut hue: u16 = 0;

    event!(
        tracing::Level::INFO,
        "Starting game loop ({:.3}ms)",
        loop_time.as_secs_f32() * 1000.0
    );

    let mut main_loop = || {
        let start = Instant::now();

        // TODO: Fix key repeat delay issues by using VecDeque for instant key repeat
        for event in event_pump.poll_iter() {
            match event {
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Hidden => {
                        event!(tracing::Level::DEBUG, "Window hidden");
                        shown = false;
                    }
                    WindowEvent::Shown => {
                        event!(tracing::Level::DEBUG, "Window shown");
                        shown = true;
                    }
                    _ => {}
                },
                // Handle quitting keys or window close
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape) | Some(Keycode::Q),
                    ..
                } => {
                    event!(tracing::Level::INFO, "Exit requested. Exiting...");
                    return false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    paused = !paused;
                    event!(
                        tracing::Level::INFO,
                        "{}",
                        if paused { "Paused" } else { "Unpaused" }
                    );
                }
                Event::KeyDown { keycode, .. } => {
                }
                _ => {}
            }
        }

        // TODO: Proper pausing implementation that does not interfere with statistic gathering
        if !paused {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            canvas.clear();

            // Draw a square under the mouse
            let mouse_state = event_pump.mouse_state();
            let mouse_x = mouse_state.x();
            let mouse_y = mouse_state.y();
            let square = sdl2::rect::Rect::new(mouse_x - 25, mouse_y - 25, 50, 50);
            
            // convert hue to rgb
            let hue_rgb = colors_transform::Hsl::from(hue as f32, 100.0, 100.0).to_rgb();
            let color = sdl2::pixels::Color::RGB(hue_rgb.get_red() as u8, hue_rgb.get_green() as u8, hue_rgb.get_blue() as u8);

            event!(
                tracing::Level::DEBUG,
                "Drawing square at ({}, {}) with color {:?}",
                mouse_x,
                mouse_y,
                color
            );

            canvas.set_draw_color(color);
            canvas.fill_rect(square).unwrap();
            
            canvas.present();

            hue += 1;
            if hue >= 360 {
                hue = 0;
            }
        }

        if start.elapsed() < loop_time {
            let time = loop_time.saturating_sub(start.elapsed());
            if time != Duration::ZERO {
                #[cfg(not(target_os = "emscripten"))]
                {
                    spin_sleep::sleep(time);
                }
                #[cfg(target_os = "emscripten")]
                {
                    std::thread::sleep(time);
                }
            }
            sleep_time += time;
        } else {
            event!(
                tracing::Level::WARN,
                "Game loop behind schedule by: {:?}",
                start.elapsed() - loop_time
            );
        }

        tick_no += 1;

        const PERIOD: u32 = 60 * 60;
        let tick_mod = tick_no % PERIOD;
        if tick_mod % PERIOD == 0 {
            let average_fps = PERIOD as f32 / last_averaging_time.elapsed().as_secs_f32();
            let average_sleep = sleep_time / PERIOD;
            let average_process = loop_time - average_sleep;

            event!(
                tracing::Level::DEBUG,
                "Timing Averages [fps={}] [sleep={:?}] [process={:?}]",
                average_fps,
                average_sleep,
                average_process
            );

            sleep_time = Duration::ZERO;
            last_averaging_time = Instant::now();
        }

        true
    };

    loop {
        if !main_loop() {
            break;
        }
    }
}
