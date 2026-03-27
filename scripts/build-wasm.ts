#!/usr/bin/env bun
import { existsSync, mkdirSync, cpSync, copyFileSync } from "node:fs";
import { join } from "node:path";
import { parseArgs } from "node:util";
import { execSync } from "node:child_process";

const { values: flags } = parseArgs({
  options: {
    release: { type: "boolean", short: "r", default: false },
    "skip-emsdk": { type: "boolean", short: "e", default: false },
    serve: { type: "boolean", short: "s", default: false },
    clean: { type: "boolean", short: "c", default: false },
  },
  strict: true,
});

function run(cmd: string) {
  console.log(`> ${cmd}`);
  execSync(cmd, { stdio: "inherit" });
}

if (flags.clean) {
  console.log("Cleaning target directory");
  run("cargo clean");
  if (existsSync("dist")) {
    run("rm -rf dist");
  }
}

if (!flags["skip-emsdk"]) {
  console.log("Activating Emscripten");
  run("../emsdk/emsdk activate 3.1.74");
  // source emsdk_env.sh equivalent — exec in a subshell for subsequent commands
  // Since we can't source in Node, we run cargo in a shell that sources first
}

const buildType = flags.release ? "release" : "debug";
const releaseFlag = flags.release ? " --release" : "";

console.log("Building WASM with Emscripten");
if (flags["skip-emsdk"]) {
  run(`cargo build --target=wasm32-unknown-emscripten${releaseFlag}`);
} else {
  // Source emsdk env and build in the same shell
  run(`bash -c 'source ../emsdk/emsdk_env.sh && cargo build --target=wasm32-unknown-emscripten${releaseFlag}'`);
}

console.log("Generating CSS");
run("bun x postcss ./assets/styles.scss -o ./assets/build.css");

console.log("Copying WASM files to dist/");
mkdirSync("dist", { recursive: true });

const outputFolder = `target/wasm32-unknown-emscripten/${buildType}`;

const staticAssets = ["index.html", "favicon.ico", "build.css"];
for (const file of staticAssets) {
  copyFileSync(join("assets", file), join("dist", file));
}

// Copy web fonts
for (const ext of [".woff", ".woff2"]) {
  const glob = new Bun.Glob(`assets/*${ext}`);
  for (const path of glob.scanSync(".")) {
    const filename = path.split("/").pop()!;
    copyFileSync(path, join("dist", filename));
  }
}

// Copy WASM artifacts
copyFileSync(join(outputFolder, "spiritus.wasm"), join("dist", "spiritus.wasm"));
copyFileSync(join(outputFolder, "spiritus.js"), join("dist", "spiritus.js"));
copyFileSync(join(outputFolder, "deps", "spiritus.data"), join("dist", "spiritus.data"));

const sourceMap = join(outputFolder, "spiritus.wasm.map");
if (existsSync(sourceMap)) {
  copyFileSync(sourceMap, join("dist", "spiritus.wasm.map"));
}

console.log("Build complete!");

if (flags.serve) {
  console.log("Serving on http://localhost:8080");
  run("python3 -m http.server -d ./dist/ 8080");
}
