# Riot Auto Accept

Lightweight Tauri app that auto-accepts ready checks for **League of Legends** and **Teamfight Tactics**.
Bilingual UI (PL / EN), gaming-style HUD, ~10 MB installer, ~30 MB RAM.

![status](https://img.shields.io/badge/status-stable-22d3ee)
![platform](https://img.shields.io/badge/platform-Windows-blue)
![license](https://img.shields.io/badge/license-MIT-green)

---

## Features

- One-click auto-accept for League and TFT ready checks
- Detects the running client via process command-line + lockfile fallback
- WebSocket + REST polling for resilience (works even if WS misses an event)
- System tray with quick toggle and show/hide
- Single-instance lock — second launch focuses the existing window
- Optional autostart with Windows (`--hidden` flag)
- Minimize-to-tray on close (configurable)
- Always-on-top toggle
- System notifications when a match is auto-accepted
- Bilingual UI with auto-detection of system locale (PL / EN)
- Persistent statistics — match accepted counter
- Tiny footprint compared to Electron alternatives

## Install

Grab the latest installer from the [Releases](https://github.com/sikorafranciszek/riot-autoaccept/releases) page.

- `Riot Auto Accept_<version>_x64-setup.exe` — NSIS installer
- Windows SmartScreen may warn (binary is unsigned) — click **More info → Run anyway**.

## Build from source

Requires Rust (stable) and Node 20+.

```bash
npm ci
npm run tauri:build
```

The bundle ends up in `src-tauri/target/release/bundle/`.

## Development

```bash
npm ci
npm run tauri:dev
```

## How it works

The League Client exposes a local REST + WebSocket API (LCU) on `127.0.0.1:<port>`
with HTTPS and Basic Auth. We:

1. Find the `LeagueClientUx` process and parse `--app-port` / `--remoting-auth-token` from its command line (`sysinfo`). Falls back to reading the client's lockfile.
2. Connect over `wss://` with a custom cert verifier (LCU uses a self-signed cert).
3. Subscribe to `OnJsonApiEvent` and watch for `/lol-matchmaking/v1/ready-check` events.
4. When a ready check fires with `state = "InProgress"` and `playerResponse = "None"`, we POST to `/lol-matchmaking/v1/ready-check/accept`.

REST polling runs alongside the WebSocket as a safety net.

---

## Polski

Lekka aplikacja Tauri która automatycznie akceptuje znalezione mecze w **League of Legends** i **Teamfight Tactics**.

### Funkcje

- Automatyczna akceptacja ready check w LoL i TFT
- Wykrywanie klienta przez proces + plik blokady jako fallback
- WebSocket + polling REST dla niezawodności
- Ikona w zasobniku systemowym z szybkim toggle
- Pojedyncza instancja
- Opcjonalny start z Windowsem
- Chowanie do zasobnika przy zamykaniu (do wyłączenia)
- Zawsze na wierzchu (opcjonalnie)
- Powiadomienia systemowe po akceptacji
- UI po polsku i angielsku z auto-detekcją języka
- Statystyki zaakceptowanych meczów

### Instalacja

Pobierz instalator z zakładki [Releases](https://github.com/sikorafranciszek/riot-autoaccept/releases).

Windows SmartScreen może ostrzec o niepodpisanym binarium — kliknij **Więcej informacji → Uruchom mimo to**.

---

## Disclaimer

This is a personal automation tool. Auto-accepting ready checks is in a gray area
of the Riot Games Terms of Service. Use at your own risk. The author takes no
responsibility for any action taken against accounts using this tool.

## License

MIT — see [LICENSE](./LICENSE).
