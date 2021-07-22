use std::error::Error;

/// Export allows you to save the network with a specific output format.
///
/// In some format all information about the network can not be kept, please refer to the
/// documentation of the exporter to know which information are dropped.
pub trait Export<N> {
    /// Export the network
    ///
    /// # Errors
    /// Should raise an error if there is a problem during the export of the network (often related
    /// to IO problems).
    fn export(&mut self, net: &N) -> Result<(), Box<dyn Error>>;
}
