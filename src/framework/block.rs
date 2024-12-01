extern crate keyboard;
extern crate tempo;

/// Struct which provides read-only access to the playback state.
pub struct PlaybackState<'a> {
    /// See tempo::Tempo
    pub tempo: &'a tempo::Tempo,

    /// See keyboard::Keyboard
    pub keyboard: &'a keyboard::Keyboard,
}

/// Block which produces audio data.
pub trait Source {
    /// Read data from the source into the output stream.
    fn read(&mut self, state: &PlaybackState);

    /// Optional code to be run when the playback is complete.
    fn cleanup(&mut self) {}

    /// Optional property for whether the block contains blocking I/O.
    fn is_blocking_io(&self) -> bool {
        return false;
    }
}

/// Block which ingests audio data and outputs it to I/O.
pub trait Sink {
    /// Write data from the input stream to the sink.
    fn write(&mut self, state: &PlaybackState);

    /// Optional code to be run when the playback is complete.
    fn cleanup(&mut self) {}

    /// Optional property for whether the block contains blocking I/O.
    fn is_blocking_io(&self) -> bool {
        return false;
    }
}

/// Block which takes an input data source and produces a transformed output.
pub trait Transformer {
    /// Transform input streams into their associated output streams.
    fn transform(&mut self, state: &PlaybackState);

    /// Optional code to be run when the playback is complete.
    fn cleanup(&mut self) {}
}
