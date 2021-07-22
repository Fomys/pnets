use std::error::Error;
use std::io::Write;

use pnets::timed::{Bound, TimeRange};
use pnets::{standard, timed, NetError};

/// Create a new dotnet exporter from parameters
pub struct ExporterBuilder<W: Write> {
    writer: W,
    without_disconnected_transitions: bool,
    with_all_places: bool,
}

impl<W> ExporterBuilder<W>
where
    W: Write,
{
    /// Create a new builder
    ///
    /// By default all disconnected transitions and places are exported
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            without_disconnected_transitions: false,
            with_all_places: false,
        }
    }
    /// Set if export should keep all disconnected transition or not
    pub fn with_disconnected_transitions(self, without_disconnected_transitions: bool) -> Self {
        Self {
            without_disconnected_transitions,
            ..self
        }
    }
    /// Set if export should contains all places or not
    pub fn with_all_places(self, with_all_places: bool) -> Self {
        Self {
            with_all_places,
            ..self
        }
    }
    /// Build the exporter
    pub fn build(self) -> Exporter<W> {
        Exporter {
            writer: self.writer,
            without_disconnected_transition: self.without_disconnected_transitions,
            with_all_places: self.with_all_places,
        }
    }
}

/// Exporter for [dotnet]() format.
///
/// It consume a network ([`pnets::standard::Net`] or [`pnets::timed::Net`]) and it write its
/// representation in the writer.
pub struct Exporter<W>
where
    W: Write,
{
    writer: W,
    without_disconnected_transition: bool,
    with_all_places: bool,
}

impl<W> Exporter<W>
where
    W: Write,
{
    fn escape(s: &str) -> String {
        format!(
            "{{{}}}",
            s.replace("\\", "\\\\")
                .replace("{", "\\{")
                .replace("}", "\\}")
        )
    }
}

impl<W> pnets::io::Export<timed::Net> for Exporter<W>
where
    W: Write,
{
    fn export(&mut self, net: &timed::Net) -> Result<(), Box<dyn Error>> {
        if !net.name.is_empty() {
            self.writer
                .write_all(format!("net {}\n", Self::escape(&net.name)).as_ref())?;
        }
        for pl in net.places.iter() {
            if self.with_all_places
                | (!pl.is_disconnected() && (!pl.label.is_empty() | (pl.initial != 0)))
            {
                self.writer
                    .write_all(format!("pl {} ", Self::escape(&pl.name)).as_ref())?;
                if !pl.label.is_empty() {
                    self.writer
                        .write_all(format!(": {} ", Self::escape(&pl.label)).as_ref())?;
                }
                if pl.initial != 0 {
                    self.writer
                        .write_all(format!("({})", pl.initial).as_ref())?;
                }
                self.writer.write_all("\n".as_ref())?;
            }
        }

        for tr in net.transitions.iter() {
            if self.without_disconnected_transition && tr.is_disconnected() {
                continue;
            }
            self.writer
                .write_all(format!("tr {} ", Self::escape(&tr.name)).as_ref())?;
            if !tr.label.is_empty() {
                self.writer
                    .write_all(format!(": {} ", Self::escape(&tr.label)).as_ref())?;
            }
            if (tr.time
                != TimeRange {
                    start: Bound::Closed(0),
                    end: Bound::Infinity,
                })
            {
                match tr.time.start {
                    Bound::Closed(v) => self.writer.write_all(format!("[{},", v).as_ref())?,
                    Bound::Open(v) => self.writer.write_all(format!("]{},", v).as_ref())?,
                    Bound::Infinity => {
                        return Err(Box::new(NetError::InvalidTimeRange));
                    }
                };
                match tr.time.end {
                    Bound::Closed(v) => self.writer.write_all(format!("{}] ", v).as_ref())?,
                    Bound::Open(v) => self.writer.write_all(format!("{}[ ", v).as_ref())?,
                    Bound::Infinity => self.writer.write_all("w[ ".as_ref())?,
                };
            }

            for &(i, w) in tr.inhibitors.iter() {
                self.writer
                    .write_all(format!("{}?-{} ", Self::escape(&net[i].name), w).as_ref())?;
            }

            for &(i, w) in tr.consume.iter() {
                match w {
                    1 => self
                        .writer
                        .write_all(format!("{} ", Self::escape(&net[i].name)).as_ref())?,
                    w => self
                        .writer
                        .write_all(format!("{}*{} ", Self::escape(&net[i].name), w).as_ref())?,
                }
            }

            for &(i, w_cond) in tr.conditions.iter() {
                self.writer
                    .write_all(format!("{}?{} ", Self::escape(&net[i].name), w_cond).as_ref())?;
                self.writer.write_all(" ".as_ref())?;
            }
            self.writer.write_all("-> ".as_ref())?;
            for &(i, w_produced) in tr.produce.iter() {
                match w_produced {
                    1 => self
                        .writer
                        .write_all(format!("{} ", Self::escape(&net[i].name)).as_ref())?,
                    w_produced => self.writer.write_all(
                        format!("{}*{} ", Self::escape(&net[i].name), w_produced).as_ref(),
                    )?,
                }
            }
            self.writer.write_all("\n".as_ref())?;
            if !tr.priorities.is_empty() {
                self.writer
                    .write_all(format!("pr {} > ", Self::escape(&tr.name)).as_ref())?;

                for &pr in &tr.priorities {
                    self.writer
                        .write_all(format!("{} ", Self::escape(&net[pr].name)).as_ref())?;
                }

                self.writer.write_all("\n".as_ref())?;
            }
        }
        Ok(())
    }
}

impl<W> pnets::io::Export<standard::Net> for Exporter<W>
where
    W: Write,
{
    fn export(&mut self, net: &standard::Net) -> Result<(), Box<dyn Error>> {
        if !net.name.is_empty() {
            self.writer
                .write_all(format!("net {}\n", Self::escape(&net.name)).as_ref())?;
        }
        for pl in net.places.iter() {
            if self.with_all_places
                | (!pl.is_disconnected() && (!pl.label.is_empty() | (pl.initial != 0)))
            {
                self.writer
                    .write_all(format!("pl {} ", Self::escape(&pl.name)).as_ref())?;
                if !pl.label.is_empty() {
                    self.writer
                        .write_all(format!(": {} ", Self::escape(&pl.label)).as_ref())?;
                }
                if pl.initial != 0 {
                    self.writer
                        .write_all(format!("({})", pl.initial).as_ref())?;
                }
                self.writer.write_all("\n".as_ref())?;
            }
        }

        for tr in net.transitions.iter() {
            if self.without_disconnected_transition && tr.is_disconnected() {
                continue;
            }
            self.writer
                .write_all(format!("tr {} ", Self::escape(&tr.name)).as_ref())?;
            if !tr.label.is_empty() {
                self.writer
                    .write_all(format!(": {} ", Self::escape(&tr.label)).as_ref())?;
            }

            for &(i, w) in tr.consume.iter() {
                match w {
                    1 => self
                        .writer
                        .write_all(format!("{} ", Self::escape(&net[i].name)).as_ref())?,
                    w => self
                        .writer
                        .write_all(format!("{}*{} ", Self::escape(&net[i].name), w).as_ref())?,
                }
            }

            self.writer.write_all("-> ".as_ref())?;

            for &(i, w_produced) in tr.produce.iter() {
                match w_produced {
                    1 => self
                        .writer
                        .write_all(format!("{} ", Self::escape(&net[i].name)).as_ref())?,
                    w_produced => self.writer.write_all(
                        format!("{}*{} ", Self::escape(&net[i].name), w_produced).as_ref(),
                    )?,
                }
            }
            self.writer.write_all("\n".as_ref())?;
        }
        Ok(())
    }
}
