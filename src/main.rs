use sdl2::event::{Event, WindowEvent};
use std::time::{Duration, Instant};
use tracing::event;
use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt;

#[cfg(target_family = "wasm")]
pub mod emscripten;

pub fn main() {
    let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();

    // Setup tracing
    let subscriber = tracing_subscriber::fmt()
        .with_ansi(cfg!(not(target_os = "emscripten")))
        .with_max_level(tracing::Level::DEBUG)
        .finish()
        .with(ErrorLayer::default());

    tracing::subscriber::set_global_default(subscriber).expect("Could not set global default");

    let window = video
        .window("SDL2 Test", 800, 800)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not initialize window");

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("Could not build canvas");

    canvas
        .set_logical_size(800, 800)
        .expect("Could not set logical size");

    // let texture_creator = canvas.texture_creator();

    let mut event_pump = ctx
        .event_pump()
        .expect("Could not get SDL EventPump");

    let loop_time = Duration::from_secs(1) / 60;
    let mut shown = true;

    // The start of a period of time over which we average the frame time.
    let mut sleep_time = Duration::ZERO;

    let mut main_loop = move || {
        let start = Instant::now();

        // TODO: Fix key repeat delay issues by using VecDeque for instant key repeat
        // for event in event_pump.poll_iter() {
        //     match event {
        //         Event::Window { win_event, .. } => match win_event {
        //             WindowEvent::Hidden => {
        //                 event!(tracing::Level::WARN, "Window hidden");
        //                 shown = false;
        //             }
        //             WindowEvent::Shown => {
        //                 event!(tracing::Level::WARN, "Window shown");
        //                 shown = true;
        //             }
        //             _ => {}
        //         },
        //         // Handle quitting keys or window close
        //         Event::KeyDown {
        //             keycode: Some(sdl2::keyboard::Keycode::Escape),
        //             ..
        //         }
        //         | Event::Quit { .. } => {
        //             event!(tracing::Level::INFO, "Exit requested. Exiting...");
        //             return;
        //         }
        //         _ => {
        //             event!(tracing::Level::WARN, "Unhandled event: {:?}", event);
        //         }
        //     }
        // }

        if shown {
            // Set background to black
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            canvas.clear();

            // // Draw a square under the mouse
            // let mouse_state = event_pump.mouse_state();
            // let mouse_x = mouse_state.x();
            // let mouse_y = mouse_state.y();
            // let square = sdl2::rect::Rect::new(mouse_x - 25, mouse_y - 25, 50, 50);

            // canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 255));
            // canvas.fill_rect(square).unwrap();

            // canvas.set_draw_color(sdl2::pixels::Color::RGBA(255, 255, 255, 50));
            // canvas
            //     .draw_line(
            //         sdl2::rect::Point::new(0, 0),
            //         sdl2::rect::Point::new(mouse_x, mouse_y),
            //     )
            //     .unwrap();

            canvas.present();

            event!(
                tracing::Level::WARN,
                "Loop took: {:?} (max {:?})",
                start.elapsed(),
                loop_time
            );

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
        }
    };

    #[cfg(target_family = "wasm")]
    use crate::emscripten;

    #[cfg(target_family = "wasm")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_family = "wasm"))]
    {
        use std::thread::sleep;
        loop {
            // main_loop(Rc::clone(&ctx), Rc::clone(&rect), Rc::clone(&canvas))();
            main_loop();
            sleep(Duration::from_millis(10))
        }
    }
}
