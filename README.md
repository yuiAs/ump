# ump

A native GUI MIDI player for Windows with SF2 soundfont synthesis.

Built with Rust, using Direct2D for hardware-accelerated rendering and a custom sequencer for low-latency audio playback.

## Features

- **SF2 Soundfont Synthesis** -- Real-time MIDI rendering via [rustysynth](https://github.com/sinshu/rustysynth)
- **Piano Roll** -- Scrolling note visualization with per-channel coloring
- **Track List** -- Two view modes: Default (16-channel grid) and Detail (per-track tree)
- **MIDI Mode Detection** -- Auto-detects GM/GS/XG/GM2 from SysEx reset messages and bank select patterns; manual override with `1`-`4` keys
- **Per-Track Mute** -- Mute/unmute individual channels during playback
- **Seek** -- Forward/backward seeking with full state reconstruction (program changes, control changes, pitch bends)
- **Custom Fonts** -- Load `.ttf`/`.otf`/`.ttc` font files via settings
- **File Browser** -- Built-in file browser for selecting MIDI and SF2 files; drive root navigation with `/` or `\`
- **Settings Persistence** -- Window geometry, volume, display mode, font, and soundfont paths saved to `settings.toml`
- **Volume Control** -- Application volume (`+`/`-`) multiplied with MIDI master volume (SysEx)

### Keybindings

| Key | Action |
|---|---|
| `Space` | Play / Pause |
| `S` | Stop |
| `Left` / `Right` | Seek -5s / +5s |
| `Up` / `Down` | Cursor Up / Down |
| `M` | Mute / Unmute track |
| `P` | Toggle piano roll |
| `E` | Toggle track view (Detail) |
| `+` / `-` | Volume Up / Down |
| `[` / `]` | Zoom Out / In |
| `Tab` | Cycle focus panel |
| `O` | Open MIDI file |
| `F` | Open SF2 file |
| `D` | Reset SF2 to default |
| `1`-`4` | Set mode (GM / GS / XG / GM2) |
| `?` | Toggle help overlay |
| `Q` / `Esc` | Quit |

## Build

### Requirements

- Rust 1.70+ (edition 2021)
- Windows 10 or later (Direct2D / DirectWrite backend)

### Commands

```sh
# Debug build
cargo build

# Release build (optimized, LTO enabled)
cargo build --release
```

### Usage

```sh
# Launch with file browser
ump

# Open a MIDI file directly
ump path/to/file.mid

# Specify a soundfont
ump path/to/file.mid --sf2 path/to/soundfont.sf2
```

## Configuration

Settings are stored at `%APPDATA%/ump/settings.toml`. See [settings.example.toml](settings.example.toml) for all available options.

Relative paths in `settings.toml` are resolved against the config directory (`%APPDATA%/ump/`).

## Limitations

- **Windows only** -- Rendering depends on Direct2D/DirectWrite. No Linux/macOS support.
- **SysEx support is partial** -- GM/GS/XG/GM2 reset messages and Master Volume are recognized. Other SysEx commands (e.g. GS part parameters, XG effect settings, drum map changes) are parsed but not reproduced by the synthesizer.
- **No MIDI output** -- Playback is software-synthesized only. External MIDI device output is not supported.
- **Single soundfont** -- Only one SF2 file can be loaded at a time. Layering multiple soundfonts is not supported.
- **No audio device selection** -- Uses the system default audio output device.

## Dependencies

| Crate | Purpose |
|---|---|
| [midly](https://crates.io/crates/midly) | MIDI file parsing |
| [rustysynth](https://crates.io/crates/rustysynth) | SF2 soundfont synthesizer |
| [cpal](https://crates.io/crates/cpal) | Cross-platform audio output |
| [winit](https://crates.io/crates/winit) | Window creation and event loop |
| [windows](https://crates.io/crates/windows) | Direct2D / DirectWrite rendering |
| [clap](https://crates.io/crates/clap) | Command-line argument parsing |
| [anyhow](https://crates.io/crates/anyhow) | Error handling |
| [serde](https://crates.io/crates/serde) / [toml](https://crates.io/crates/toml) | Settings serialization |
| [dirs](https://crates.io/crates/dirs) | Platform config directory resolution |

## License

[MIT](LICENSE)
