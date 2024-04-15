# rust-sdl2-emscripten

This is an experimental repository while testing a Rust + SDL2 project built with Emscripten.

- [hello-rust-sdl2-wasm](https://github.com/awwsmm/hello-rust-sdl2-wasm) - A bit of a weird repository, I'm not sure that the creator knows Rust that well, but it compiles. Note that the `asmjs-unknown-emscripten` target is deprecated, and you should use `wasm32-unknown-emscripten` instead. You'll need to change all the files, flags etc. to make it match.

- [build.sh](./scripts/build.sh) - One of the core files in this repository, it builds the project with Emscripten. Note all the flags available for modifying the behavior of the build.

### Goals

- [X] Reproducible SDL2 Emscripten Builds
  - This ensures that the project can iterate safely and be inspected in a safe environment, free from errors. Helps ensure errors are isolated to the machine or build script.
- [ ] SDL2 Extensions
  - [ ] Image
  - [ ] Mixer
  - [ ] TTF
  - [ ] GFX
  - All of these libraries are common and necessary for a lot of projects. Ensuring they work is important.
- [ ] Example of External Javascript Interop
  - The basic ability to provide some kind of Javascript binding would be important for a decent web-based project or game.
  - The ability to use localStorage, fetch, or some browser-only API would be important.
- [ ] Windows, MacOS, Linux Builds
  - Simple ability to provide multi-platform builds in addition to the WASM build.

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