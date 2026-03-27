import { defineConfig, presets } from "@xevion/tempo";

export default defineConfig({
  subsystems: {
    game: {
      ...presets.rust(),
      aliases: ["g", "rust"],
      commands: {
        ...presets.rust().commands,
        test: "cargo nextest run --no-fail-fast --no-tests=pass",
      },
    },
  },
  check: {
    autoFixStrategy: "fix-first",
  },
});
