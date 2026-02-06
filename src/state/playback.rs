//! Playback command enum for UI -> audio thread communication.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PlaybackCommand {
    Play,
    Pause,
    Stop,
    SeekForward,
    SeekBackward,
    VolumeUp,
    VolumeDown,
}
