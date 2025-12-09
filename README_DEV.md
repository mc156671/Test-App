# Development: Achero‑like (Rust + WebAssembly)

Kurzanleitung, um lokal mit dem Projekt zu starten.

Voraussetzungen:
- Rust toolchain (stable) und `cargo`
- `wasm-pack` (https://rustwasm.github.io/wasm-pack/installer/)
- Ein einfacher statischer Webserver (z. B. `python -m http.server` oder `basic-http-server`)

Build & Run (lokal):

```bash
# aus Projektwurzel
wasm-pack build --target web
cd static
# einen einfachen Server starten (z. B. Python)
python3 -m http.server 8000
# im Browser öffnen: http://localhost:8000
```

Was dieses Scaffold liefert:
- `src/lib.rs`: Basis‑WASM‑Entry, erstellt ein Canvas und startet eine einfache Loop
- `static/index.html`: Lädt das generierte `pkg/`-Modul (nach `wasm-pack build`)

Nächste Schritte:
- Stabilen Game‑Loop implementieren (entities, delta time)
- Input (Keyboard + Touch) korrekt handhaben
- Render‑Abstraktion, Sprites/Assets‑Loader
- Prozedurale Levelgenerierung, Gegner-Logik und Balancing
- Release‑Packaging (minify, gz, caching, CI)
