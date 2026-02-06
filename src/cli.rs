//! Command-line argument definitions.

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ump", about = "TUI MIDI Player")]
pub struct Args {
    /// Path to the MIDI file (.mid/.midi)
    pub midi_file: Option<String>,

    /// Path to the SoundFont file (.sf2)
    #[arg(long = "sf2")]
    pub sf2_file: Option<String>,
}
