use std::error::Error;

/// Parse allows you to load the network from a specific reader
///
/// In some format all information about the network can not be read because this framework doesn't
/// have a full support, please refer to the documentation of the exporter to know which information
/// are dropped.
pub trait Parse<N> {
    /// Export the network
    ///
    /// # Errors
    /// Should raise an error if there is a problem during the export of the network (often related
    /// to IO problems).
    fn parse(self) -> Result<N, Box<dyn Error>>;
}
