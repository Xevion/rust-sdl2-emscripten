# rust-sdl2-emscripten

This is an experimental repository while testing a Rust + SDL2 project built with Emscripten.

- [hello-rust-sdl2-wasm](https://github.com/awwsmm/hello-rust-sdl2-wasm) - A bit of a weird repository, I'm not sure that the creator knows Rust that well, but it compiles. Note that the `asmjs-unknown-emscripten` target is deprecated, and you should use `wasm32-unknown-emscripten` instead. You'll need to change all the files, flags etc. to make it match.

- [build.sh](./scripts/build.sh) - One of the core files in this repository, it builds the project with Emscripten. Note all the flags available for modifying the behavior of the build.

### Goals

- [X] Automatic Builds
  - [X] Web Assembly (Emscripten)
  - [ ] Windows
  - [ ] MacOS
  - [ ] Linux
  - This ensures that the project can iterate safely and be inspected in a safe environment, free from errors. Helps ensure errors are isolated to the machine or build script.
- [ ] SDL2 Extensions
  - [X] Image
  - [X] Mixer
  - [X] TTF
  - [ ] GFX
  - All of these libraries are common and necessary for a lot of projects. Ensuring they work is important.
- [ ] Example of External Javascript Interop
  - The basic ability to provide some kind of Javascript binding would be important for a decent web-based project or game.
  - The ability to use localStorage, fetch, or some browser-only API would be important.

### Concept

- A decent codebase without extras or warnings. Straightforward build process for Windows, Linux, and WASM.
  - Note: Cross-compiling for Windows is a bit of a pain, but it's possible. That said, GitHub Actions can handle Windows builds natively.
  - While SDL2 has annoying as fuck lifetimes, Emscripten callback loop imbibes even worse static lifetimes that are damn near impossible to satisfy.
- A simple example of a game loop, input handling, and rendering.
  - Pausing functionality, native-only quit.
  - FPS counter toggle (TTF example).
- Sprites with Atlas
- Resizable Canvas
- Javascript Interop
  - LocalStorage
  - Fetch

### Resources

- [KyleMiles/Rust-SDL-Emscripten-Template/](https://github.com/KyleMiles/Rust-SDL-Emscripten-Template/)
  - Has some special javascript interop code
- [gregbuchholz/RuSDLem](https://github.com/gregbuchholz/RuSDLem)
  - One of the few with a [demo](https://gregbuchholz.github.io/) available.
- [tung/ruggrogue](https://github.com/tung/ruggrogue/)
  - A very large game example, great codebase, documentation, online player.
- [arskiy/chess](https://github.com/arskiy/chess/)
  - Image usage, decent code example
  - Has more advanced javascript config and examples to look at.
- [deckarep/flappy-rust](https://github.com/deckarep/flappy-rust/)
  - Image + Mixer Usage, possibly GFX & TTF
- [aelred/tetris](https://github.com/aelred/tetris)
  - Contains multiple sub-projects, including a web server. Uses SDL2 TTF & Mixer.
  - Most recent development (3 months ago).
  - Custom font loading, packed inside the binary (WASM) instead of `.data` file, or external file.
  - Advanced Emscripten bindings for Javascript (fetch, GET/POST)
  - No Asyncify, uses `emscripten_set_main_loop` callback instead.
  - See the [REST functions](https://github.com/aelred/tetris/blob/master/tetris/src/rest.rs#L99) for Emscripten.
- [coderedart/rust-sdl2-wasm](https://github.com/coderedart/rust-sdl2-wasm/tree/master)
  - This is mostly interesting because it has an egui implementation; egui is very cool for demos, developer tooling, debug menus, and so on.
  - The only thing I don't understand is where SDL2 is; there is almost no real code referencing SDL2 except a `SDL2Backend` provided by the `egui` crate. Weird.
  - While devoid of anything particularly interesting for my own needs, it has a demo [here](https://coderedart.github.io/rust-sdl2-wasm/)
