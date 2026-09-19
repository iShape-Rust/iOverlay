# Overlay Editor

Native and WebAssembly editor built with `eframe`/`egui`. The existing Boolean,
String, Stroke, Variable Stroke, and Outline modes share the original JSON fixtures.

## Native

From this directory:

```sh
cargo run --release
```

Fixtures are resolved relative to `CARGO_MANIFEST_DIR`, so running with
`cargo run --manifest-path /path/to/overlay_editor/Cargo.toml` also works.

- Drag a point to edit its integer coordinates.
- Drag empty canvas space to pan.
- Scroll over the canvas to zoom around the pointer.
- Use Up/Down to change fixtures when a parameter control does not have focus.

## WebAssembly

```sh
cargo check --target wasm32-unknown-unknown --lib
wasm-pack build --release --target web
```

The exported `WebApp` keeps its five JSON input strings. Startup now returns a
Promise; keep the instance alive and await startup to handle graphics errors:

```js
import init, { WebApp } from "./pkg/overlay_editor.js";
await init();
const app = new WebApp();
await app.start(booleanJson, stringJson, strokeJson, variableStrokeJson, outlineJson);
// app.destroy() releases the runner when removing the editor.
```

The runner uses `<canvas id="overlay-editor-canvas">`. If it is absent, startup
creates a full-window canvas in the document body. An existing canvas can be sized
by the embedding page.

## Rendering and layout

`i_triangle` remains a local path dependency and uses the local `i_overlay`
checkout through its own manifest. It triangulates polygon fills, including holes.
The local `i_mesh` dependency builds contours, open paths, and arrows with
`RoundStrokeBuilder` (round joins and caps), rendered as `egui::Mesh`. Point markers
and the grid use native egui painting APIs. `iced` is no longer required.

The original `app/*/{content,control,workspace}`, `draw`, `geom`, `sheet`,
`point_editor`, and `data` modules are retained. Parameters occupy the area above
the canvas; geometry and fixture calculations remain in the content modules.

## Checks

```sh
cargo test --lib
cargo check --all-targets
cargo check --target wasm32-unknown-unknown --lib
```

Tests cover all fixture workspaces, Boolean/String modes, polygon holes,
degenerate strokes, point dragging, panning, zoom anchoring, and arrow navigation.
