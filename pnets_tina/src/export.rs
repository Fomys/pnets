use std::error::Error;
use std::io::Write;

use pnets::timed::{Bound, TimeRange};
use pnets::{timed, NetError, NodeId};

/// Create a new tina exporter from parameters
pub struct ExporterBuilder<'w> {
    writer: &'w mut dyn Write,
    without_disconnected_transitions: bool,
    with_all_places: bool,
}

impl<'w> ExporterBuilder<'w> {
    /// Create a new builder
    ///
    /// By default all disconnected transitions and places are exported
    pub fn new(writer: &'w mut dyn Write) -> Self {
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
    pub fn build(self) -> Exporter<'w> {
        Exporter {
            writer: self.writer,
            without_disconnected_transition: self.without_disconnected_transitions,
            with_all_places: self.with_all_places,
        }
    }
}

/// Exporter for [tina]() format.
///
/// It consume a network ([`pnets::timed::Net`]) and it write its
/// representation in the writer.
pub struct Exporter<'w> {
    writer: &'w mut dyn Write,
    without_disconnected_transition: bool,
    with_all_places: bool,
}

impl<'w> Exporter<'w> {
    fn escape(s: &str) -> String {
        format!(
            "{{{}}}",
            s.replace("\\", "\\\\")
                .replace("{", "\\{")
                .replace("}", "\\}")
        )
    }

    /// Export a timed net
    pub fn export(&mut self, net: &timed::Net) -> Result<(), Box<dyn Error>> {
        if !net.name.is_empty() {
            self.writer
                .write_all(format!("net {}\n", Self::escape(&net.name)).as_ref())?;
        }
        for (pl, place) in net.places.iter_enumerated() {
            if self.with_all_places
                | (!place.is_disconnected() && (place.label.is_some() | (place.initial != 0)))
            {
                self.writer.write_all(
                    format!(
                        "pl {} ",
                        Self::escape(&net.get_name_by_index(&NodeId::Place(pl)).unwrap())
                    )
                    .as_ref(),
                )?;
                if place.label.is_some() {
                    self.writer.write_all(
                        format!(": {} ", Self::escape(place.label.as_ref().unwrap())).as_ref(),
                    )?;
                }
                if place.initial != 0 {
                    self.writer
                        .write_all(format!("({})", place.initial).as_ref())?;
                }
                self.writer.write_all("\n".as_ref())?;
            }
        }

        for (tr, transition) in net.transitions.iter_enumerated() {
            if self.without_disconnected_transition && transition.is_disconnected() {
                continue;
            }
            self.writer.write_all(
                format!(
                    "tr {} ",
                    Self::escape(&net.get_name_by_index(&NodeId::Transition(tr)).unwrap())
                )
                .as_ref(),
            )?;
            if transition.label.is_some() {
                self.writer.write_all(
                    format!(": {} ", Self::escape(transition.label.as_ref().unwrap())).as_ref(),
                )?;
            }
            if (transition.time
                != TimeRange {
                    start: Bound::Closed(0),
                    end: Bound::Infinity,
                })
            {
                match transition.time.start {
                    Bound::Closed(v) => self.writer.write_all(format!("[{},", v).as_ref())?,
                    Bound::Open(v) => self.writer.write_all(format!("]{},", v).as_ref())?,
                    Bound::Infinity => {
                        return Err(Box::new(NetError::InvalidTimeRange));
                    }
                };
                match transition.time.end {
                    Bound::Closed(v) => self.writer.write_all(format!("{}] ", v).as_ref())?,
                    Bound::Open(v) => self.writer.write_all(format!("{}[ ", v).as_ref())?,
                    Bound::Infinity => self.writer.write_all("w[ ".as_ref())?,
                };
            }

            for &(pl, w) in transition.inhibitors.iter() {
                self.writer.write_all(
                    format!(
                        "{}?-{} ",
                        Self::escape(&net.get_name_by_index(&NodeId::Place(pl)).unwrap()),
                        w
                    )
                    .as_ref(),
                )?;
            }

            for &(pl, w) in transition.consume.iter() {
                match w {
                    1 => self.writer.write_all(
                        format!(
                            "{} ",
                            Self::escape(&net.get_name_by_index(&NodeId::Place(pl)).unwrap())
                        )
                        .as_ref(),
                    )?,
                    w => self.writer.write_all(
                        format!(
                            "{}*{} ",
                            Self::escape(&net.get_name_by_index(&NodeId::Place(pl)).unwrap()),
                            w
                        )
                        .as_ref(),
                    )?,
                }
            }

            for &(pl, w_cond) in transition.conditions.iter() {
                self.writer.write_all(
                    format!(
                        "{}?{} ",
                        Self::escape(&net.get_name_by_index(&NodeId::Place(pl)).unwrap()),
                        w_cond
                    )
                    .as_ref(),
                )?;
                self.writer.write_all(" ".as_ref())?;
            }
            self.writer.write_all("-> ".as_ref())?;
            for &(pl, w_produced) in transition.produce.iter() {
                match w_produced {
                    1 => self.writer.write_all(
                        format!(
                            "{} ",
                            Self::escape(&net.get_name_by_index(&NodeId::Place(pl)).unwrap()),
                        )
                        .as_ref(),
                    )?,
                    w_produced => self.writer.write_all(
                        format!(
                            "{}*{} ",
                            Self::escape(&net.get_name_by_index(&NodeId::Place(pl)).unwrap()),
                            w_produced
                        )
                        .as_ref(),
                    )?,
                }
            }
            self.writer.write_all("\n".as_ref())?;
            if !transition.priorities.is_empty() {
                self.writer.write_all(
                    format!(
                        "pr {} > ",
                        Self::escape(&net.get_name_by_index(&NodeId::Transition(tr)).unwrap()),
                    )
                    .as_ref(),
                )?;

                for &pr in &transition.priorities {
                    self.writer.write_all(
                        format!(
                            "{} ",
                            Self::escape(&net.get_name_by_index(&NodeId::Transition(pr)).unwrap())
                        )
                        .as_ref(),
                    )?;
                }

                self.writer.write_all("\n".as_ref())?;
            }
        }
        Ok(())
    }
}
