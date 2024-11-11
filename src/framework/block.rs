/// Block which produces audio data.
pub trait Source {
    /// Read data from the source into the output stream.
    fn read(&mut self);
}

/// Block which ingests audio data and outputs it to I/O.
pub trait Sink {
    /// Write data from the input stream to the sink.
    fn write(&mut self);
}

/// Block which takes an input data source and produces a transformed output.
pub trait Transformer {
    /// Transform input streams into their associated output streams.
    fn transform(&mut self);
}
