use std::time::Duration;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::mixer;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::rwops::RWops;
use sdl2::ttf;

static MUSIC_DATA: &[u8] = include_bytes!("../assets/tetris.ogg");
static BLACK: Color = Color::RGB(0, 0, 0);

#[cfg(target_os = "emscripten")]
mod emscripten;

mod store;

#[cfg(not(target_os = "emscripten"))]
fn sleep(ms: u32) {
    std::thread::sleep(Duration::from_millis(ms as u64));
}

#[cfg(target_os = "emscripten")]
fn now() -> f64 {
    emscripten::now() / 1000f64
}

#[cfg(not(target_os = "emscripten"))]
fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

#[cfg(target_os = "emscripten")]
fn sleep(ms: u32) {
    emscripten::sleep(ms);
}

#[cfg(not(target_os = "emscripten"))]
fn ttf_context() -> ttf::Sdl2TtfContext {
    ttf::init().expect("failed to initialize SDL2_ttf")
}

// Deliberately leak so we get a static lifetime on Emscripten.
// See https://github.com/aelred/tetris/blob/0ad88153db/tetris-sdl/src/main.rs#L88-L92
#[cfg(target_os = "emscripten")]
fn ttf_context() -> &'static ttf::Sdl2TtfContext {
    Box::leak(Box::new(ttf::init().expect("failed to initialize SDL2_ttf")))
}

#[cfg(not(target_os = "windows"))]
fn hide_console_window() {}

#[cfg(target_os = "windows")]
fn hide_console_window() {
    use std::io::IsTerminal;
    if !std::io::stdout().is_terminal() {
        unsafe { winapi::um::wincon::FreeConsole() };
    }
}

struct GameState {
    position: Point,
    mouse: Point,
    window_size: Point,
    frame: i32,
    volume: u32,
    previous_focus: bool,
    running: bool,
    storage: store::Store,
    prev_time: f64,
    frame_count: u64,
    avg_fps: f64,
}

impl GameState {
    fn new(window_size: Point) -> Self {
        let mut storage = store::Store::new();
        let volume = storage.volume().unwrap_or_else(|| {
            println!("No volume stored, using default");
            1
        });
        println!("Volume: {}", volume);
        mixer::Music::set_volume(volume as i32);

        Self {
            position: Point::new(0, 0),
            mouse: Point::new(0, 0),
            window_size,
            frame: 0,
            volume,
            previous_focus: false,
            running: true,
            storage,
            prev_time: now(),
            frame_count: 0,
            avg_fps: 0.0,
        }
    }

    fn handle_event(&mut self, event: Event, keyboard_mod: sdl2::keyboard::Mod) -> bool {
        let mut moved = false;
        match event {
            Event::MouseMotion { x, y, .. } => {
                self.mouse = Point::new(x, y);
            }
            Event::Window {
                win_event: sdl2::event::WindowEvent::Resized(w, h),
                ..
            } => {
                println!("Resized to {}x{}", w, h);
                self.window_size.x = w;
                self.window_size.y = h;
            }
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::ESCAPE),
                ..
            } => {
                self.running = false;
            }
            Event::KeyDown { keycode: Some(key), .. } => match key {
                Keycode::SPACE => {
                    self.frame = (self.frame + 1) % 8;
                }
                Keycode::LEFT => {
                    self.position.x -= 32;
                    moved = true;
                }
                Keycode::RIGHT => {
                    self.position.x += 32;
                    moved = true;
                }
                Keycode::UP => {
                    if keyboard_mod.contains(sdl2::keyboard::Mod::LSHIFTMOD) {
                        if self.volume < 128 {
                            self.volume += 1;
                            mixer::Music::set_volume(self.volume as i32);
                            self.storage.set_volume(self.volume);
                        }
                    } else {
                        self.position.y -= 32;
                        moved = true;
                    }
                }
                Keycode::DOWN => {
                    if keyboard_mod.contains(sdl2::keyboard::Mod::LSHIFTMOD) {
                        if self.volume > 0 {
                            self.volume -= 1;
                            mixer::Music::set_volume(self.volume as i32);
                            self.storage.set_volume(self.volume);
                        }
                    } else {
                        self.position.y += 32;
                        moved = true;
                    }
                }
                _ => {}
            },
            _ => {}
        }
        moved
    }

    fn wrap_position(&mut self, canvas_size: (u32, u32)) {
        if self.position.x < 0 {
            self.position.x = canvas_size.0 as i32 - 32;
        } else if self.position.x >= canvas_size.0 as i32 {
            self.position.x = 0;
        }
        if self.position.y < 0 {
            self.position.y = canvas_size.1 as i32 - 32;
        } else if self.position.y >= canvas_size.1 as i32 {
            self.position.y = 0;
        }
    }

    fn update_fps(&mut self) {
        let duration = Duration::from_secs_f64(now() - self.prev_time);
        let fps = 1.0 / duration.as_secs_f64();
        self.prev_time = now();

        self.frame_count += 1;
        let a = 1.0 / self.frame_count as f64;
        self.avg_fps = a * fps + (1.0 - a) * self.avg_fps;
    }

    fn update_focus(&mut self, focused: bool) {
        if focused != self.previous_focus {
            if focused {
                println!("Focus gained");
            } else {
                println!("Focus lost");
            }
            self.previous_focus = focused;
        }
    }
}

fn main() {
    hide_console_window();

    let ctx = sdl2::init().expect("failed to initialize SDL2");
    let video_ctx = ctx.video().expect("failed to initialize SDL2 video");
    let window_size = Point::new(640, 480);

    let window = video_ctx
        .window("rust-sdl2-emscripten", window_size.x as u32, window_size.y as u32)
        .position_centered()
        .resizable()
        .allow_highdpi()
        .opengl()
        .build()
        .expect("failed to create window");

    let mut canvas = window.into_canvas().accelerated().build().expect("failed to create canvas");

    let texture_creator = canvas.texture_creator();
    let ttf_ctx = ttf_context();

    mixer::open_audio(44_100, sdl2::mixer::AUDIO_S16LSB, sdl2::mixer::DEFAULT_CHANNELS, 1024)
        .expect("failed to open audio device");

    let mut state = GameState::new(window_size);

    let music_data = RWops::from_bytes(MUSIC_DATA).expect("failed to load music data");
    let music = mixer::LoaderRWops::load_music(&music_data).expect("failed to load music");
    music.play(-1).expect("failed to play music");

    let font = ttf_ctx.load_font("./assets/TerminalVector.ttf", 12).expect("failed to load font");
    let fruit_atlas = texture_creator.load_texture("./assets/fruit.png").expect("could not load texture");

    let target_fps = 60;
    let frame_time = Duration::from_secs_f32(1.0) / target_fps;

    while state.running {
        let start = now();
        let mut moved = false;

        let keyboard_mod = ctx.keyboard().mod_state();
        for event in ctx.event_pump().unwrap().poll_iter() {
            moved |= state.handle_event(event, keyboard_mod);
        }

        if moved {
            let canvas_size = canvas.window().size();
            state.wrap_position(canvas_size);
        }

        canvas.set_draw_color(BLACK);
        canvas.clear();

        let focused = ctx.mouse().focused_window_id().is_some();
        state.update_focus(focused);

        if focused {
            let mouse_x = (state.mouse.x / 32 * 32) as i16;
            let mouse_y = (state.mouse.y / 32 * 32) as i16;
            let color = Color::RGB(255, 255, 255);
            let _ = canvas.line(mouse_x, mouse_y, mouse_x + 32, mouse_y, color);
            let _ = canvas.line(mouse_x + 32, mouse_y, mouse_x + 32, mouse_y + 32, color);
            let _ = canvas.line(mouse_x + 32, mouse_y + 32, mouse_x, mouse_y + 32, color);
            let _ = canvas.line(mouse_x, mouse_y + 32, mouse_x, mouse_y, color);
        }

        canvas
            .copy_ex(
                &fruit_atlas,
                Rect::new(32 * state.frame, 0, 32, 32),
                Rect::new(state.position.x, state.position.y, 32, 32),
                0.0,
                Some(Point::new(0, 0)),
                false,
                false,
            )
            .unwrap();

        let text = format!("{:.0}", state.avg_fps);
        let surface = font.render(&text).blended(Color::RGBA(255, 255, 255, 50)).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let _ = canvas.copy(
            &texture,
            None,
            Rect::new(state.window_size.x - (25i32 * text.len() as i32), 0, 25 * text.len() as u32, 40),
        );

        canvas.present();

        let t2 = now();
        let elapsed = Duration::from_secs_f64(t2 - start);
        if elapsed < frame_time {
            let sleep_time = frame_time - elapsed;
            sleep(sleep_time.as_millis() as u32);
        } else {
            let excess = elapsed - frame_time;
            println!("! excess: {:?} ({:?})", excess, t2);
        }

        state.update_fps();
    }
}
