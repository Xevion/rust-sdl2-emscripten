# rust-sdl2-emscripten

A [demo](https://xevion.github.io/rust-sdl2-emscripten/) project to explore Rust + SDL2 + Emscripten.

- Cross-platform build scripts with GitHub Actions for Windows, Linux, and Web Assembly.
- All SDL2 extensions enabled (Image, Mixer, TTF, GFX) and used in the project.
- Emscripten: Asyncify for the game loop, Javascript Interop, `extern` functions & hooks.

### Goals

- [ ] Emscrimpten Looping Method
  - [X] Asyncify
  - [ ] `emscripten_set_main_loop`
- [ ] Automatic Builds
  - [X] Web Assembly (Emscripten)
  - [ ] Windows
  - [ ] Linux
  - [ ] MacOS
- [X] SDL2 Extensions
  - [X] Image
  - [X] Mixer
  - [X] TTF
  - [X] GFX
- [X] Javascript Interop
  - [X] LocalStorage
  - [ ] Fetch
- [ ] Resizable Window

### Considerations

This project took a lot of work to get 'right', and it's still quite hacky.

- It's not possible to quickly get accurate non-monotonic timings with Emscripten, as `Instant::now()` is not broken (seconds only), and `emscripten_get_now()` is only accurate to milliseconds. The FPS counter shows inconsistent (but stable) readings on different browsers for this reason.
- `emscripten_set_main_loop` was quite difficult for me to use due to static lifetime issues, perhaps this is just a skill issue on my part, but I found it easier to use `Asyncify` initially.

If you're new to Rust + SDL2 and are interested in Emscripten, I would recommend reconsidering your need for Web builds, focusing entirely on Web-only, or using a different language/framework. Native C++ with SDL2 is likely more stable and easier to work with.

While this combination and project is possible, it's not easy, documentation/examples are sparse, and the tooling is not as mature as other languages.

My worry with a bigger project is that the complexity of the system will grow exponentially, and the time spent on debugging and fixing issues will be much higher than the time spent on actual development.

### Resources

A list of various resources I relied on and studied while building this project. Organized in descending 'usefulness'.

- [build.sh](./scripts/build.sh) - One of the core files in this repository, it builds the project with Emscripten. Note all the flags available for modifying the behavior of the build.
- [aelred/tetris](https://github.com/aelred/tetris)
  - Contains multiple sub-projects, including a web server. Uses SDL2 TTF & Mixer.
  - Most recent development (3 months ago).
  - Custom font loading, packed inside the binary (WASM) instead of `.data` file, or external file.
  - Advanced Emscripten bindings for Javascript (fetch, GET/POST)
  - No Asyncify, uses `emscripten_set_main_loop` callback instead.
  - See the [REST functions](https://github.com/aelred/tetris/blob/master/tetris/src/rest.rs#L99) for Emscripten.
- [gregbuchholz/RuSDLem](https://github.com/gregbuchholz/RuSDLem)
  - One of the few with a [demo](https://gregbuchholz.github.io/) available.
- [tung/ruggrogue](https://github.com/tung/ruggrogue/)
  - A very large game example, great codebase, documentation, online player.
- [KyleMiles/Rust-SDL-Emscripten-Template/](https://github.com/KyleMiles/Rust-SDL-Emscripten-Template/)
  - Has some special javascript interop code
- [hello-rust-sdl2-wasm](https://github.com/awwsmm/hello-rust-sdl2-wasm)
-  A bit of a weird repository, I'm not sure that the creator knows Rust that well, but it compiles.
-  Note that the `asmjs-unknown-emscripten` target is deprecated, and you should use `wasm32-unknown-emscripten` instead. You'll need to change all the files, flags etc. to make it match.
- [arskiy/chess](https://github.com/arskiy/chess/)
  - Image usage, decent code example
  - Has more advanced javascript config and examples to look at.
- [deckarep/flappy-rust](https://github.com/deckarep/flappy-rust/)
  - Image + Mixer Usage, possibly GFX & TTF
- [coderedart/rust-sdl2-wasm](https://github.com/coderedart/rust-sdl2-wasm/tree/master)
  - This is mostly interesting because it has an egui implementation; egui is very cool for demos, developer tooling, debug menus, and so on.
  - The only thing I don't understand is where SDL2 is; there is almost no real code referencing SDL2 except a `SDL2Backend` provided by the `egui` crate. Weird.
  - While devoid of anything particularly interesting for my own needs, it has a demo [here](https://coderedart.github.io/rust-sdl2-wasm/)
