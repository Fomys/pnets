use serde::{Deserialize, Serialize, Serializer};
use std::iter::FilterMap;
use std::slice::Iter;

pub type Decimal = f64;
pub type Color = String;
#[derive(Deserialize)]
pub struct PositiveDecimal(pub f64);

impl Serialize for PositiveDecimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;
        if self.0 > 999.9 {
            Err(S::Error::custom("Value must be less than 1000"))
        } else if self.0 < 0.0 {
            Err(S::Error::custom("Value must be positive"))
        } else {
            serializer.serialize_str(&format!("{:.1}", self.0))
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct NotNul {
    #[serde(rename = "$value")]
    pub value: usize,
}

#[derive(Serialize, Deserialize)]
pub struct PositiveInteger {
    #[serde(rename = "$value")]
    pub(crate) value: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleText {
    #[serde(rename = "$value")]
    pub text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "position")]
pub struct Position {
    pub x: Decimal,
    pub y: Decimal,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "offset")]
pub struct Offset {
    pub x: Decimal,
    pub y: Decimal,
}

#[derive(Deserialize)]
pub enum Shape {
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "curve")]
    Curve,
}

impl Serialize for Shape {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            Self::Line => "line",
            Self::Curve => "curve",
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "line")]
pub struct Line {
    pub shape: Option<Shape>,
    pub color: Option<Color>,
    pub width: Option<PositiveDecimal>,
}

#[derive(Deserialize)]
pub enum Rotation {
    #[serde(rename = "vertical")]
    Vertical,
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "diagonal")]
    Diagonal,
}

impl Serialize for Rotation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            Rotation::Vertical => "vertical",
            Rotation::Horizontal => "horizontal",
            Rotation::Diagonal => "diagonal",
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "fill")]
pub struct Fill {
    pub color: Option<Color>,
    #[serde(rename = "gradient-color")]
    pub gradient_color: Option<Color>,
    #[serde(rename = "gradient-rotation")]
    pub gradient_rotation: Option<Rotation>,
    pub image: String,
}

#[derive(Deserialize)]
pub enum Decoration {
    #[serde(rename = "underline")]
    Underline,
    #[serde(rename = "overline")]
    Overline,
    #[serde(rename = "line-through")]
    LineThrough,
}

impl Serialize for Decoration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            Decoration::Underline => "underline",
            Decoration::Overline => "overline",
            Decoration::LineThrough => "line-through",
        })
    }
}

#[derive(Deserialize)]
pub enum Align {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "right")]
    Right,
}

impl Serialize for Align {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            Align::Left => "left",
            Align::Center => "center",
            Align::Right => "right",
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "font")]
pub struct Font {
    pub family: Option<String>,
    pub style: Option<String>,
    pub weight: Option<String>,
    pub size: Option<String>,
    pub decoration: Option<Decoration>,
    pub align: Option<Align>,
    pub rotation: Option<Decimal>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "graphics")]
pub struct AnnotationGraphics {
    pub offset: Offset,
    pub fill: Option<Fill>,
    pub font: Option<Font>,
    pub line: Option<Line>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "graphics")]
pub struct EdgeGraphics {
    #[serde(rename = "position", default)]
    positions: Vec<Position>,
    line: Option<Line>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "graphics")]
pub struct NodeGraphics {
    pub position: Position,
    pub dimension: Option<Dimension>,
    pub fill: Option<Fill>,
    pub line: Option<Line>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "dimension")]
pub struct Dimension {
    pub x: PositiveDecimal,
    pub y: PositiveDecimal,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "name")]
pub struct Name {
    pub text: SimpleText,
    pub graphics: Option<AnnotationGraphics>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "referencePlace")]
pub struct PlaceReference {
    pub id: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub name: Option<Name>,
    pub graphics: Option<NodeGraphics>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "referencePlace")]
pub struct TransitionReference {
    pub id: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub name: Option<Name>,
    pub graphics: Option<NodeGraphics>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "page")]
pub struct Page<Place, Transition, Arc> {
    #[serde(rename = "$value")]
    pub(crate) items: Vec<PageItem<Place, Transition, Arc>>,
}

impl<Place, Transition, Arc> Page<Place, Transition, Arc> {
    pub(crate) fn pages(
        &self,
    ) -> FilterMap<
        Iter<'_, PageItem<Place, Transition, Arc>>,
        fn(&PageItem<Place, Transition, Arc>) -> Option<&Page<Place, Transition, Arc>>,
    > {
        self.items.iter().filter_map(|i| {
            if let PageItem::Page(pa) = i {
                Some(pa)
            } else {
                None
            }
        })
    }
    pub(crate) fn places(
        &self,
    ) -> FilterMap<
        Iter<'_, PageItem<Place, Transition, Arc>>,
        fn(&PageItem<Place, Transition, Arc>) -> Option<&Place>,
    > {
        self.items.iter().filter_map(|i| {
            if let PageItem::Place(pa) = i {
                Some(pa)
            } else {
                None
            }
        })
    }
    pub(crate) fn transitions(
        &self,
    ) -> FilterMap<
        Iter<'_, PageItem<Place, Transition, Arc>>,
        fn(&PageItem<Place, Transition, Arc>) -> Option<&Transition>,
    > {
        self.items.iter().filter_map(|i| {
            if let PageItem::Transition(pa) = i {
                Some(pa)
            } else {
                None
            }
        })
    }
    pub(crate) fn place_references(
        &self,
    ) -> FilterMap<
        Iter<'_, PageItem<Place, Transition, Arc>>,
        fn(&PageItem<Place, Transition, Arc>) -> Option<&PlaceReference>,
    > {
        self.items.iter().filter_map(|i| {
            if let PageItem::PlaceReference(pa) = i {
                Some(pa)
            } else {
                None
            }
        })
    }
    pub(crate) fn transition_references(
        &self,
    ) -> FilterMap<
        Iter<'_, PageItem<Place, Transition, Arc>>,
        fn(&PageItem<Place, Transition, Arc>) -> Option<&TransitionReference>,
    > {
        self.items.iter().filter_map(|i| {
            if let PageItem::TransitionReference(pa) = i {
                Some(pa)
            } else {
                None
            }
        })
    }
    pub(crate) fn arcs(
        &self,
    ) -> FilterMap<
        Iter<'_, PageItem<Place, Transition, Arc>>,
        fn(&PageItem<Place, Transition, Arc>) -> Option<&Arc>,
    > {
        self.items.iter().filter_map(|i| {
            if let PageItem::Arc(pa) = i {
                Some(pa)
            } else {
                None
            }
        })
    }
}

#[derive(Serialize, Deserialize)]
pub enum PageItem<Place, Transition, Arc> {
    #[serde(rename = "page")]
    Page(Page<Place, Transition, Arc>),
    #[serde(rename = "place")]
    Place(Place),
    #[serde(rename = "transition")]
    Transition(Transition),
    #[serde(rename = "referencePlace")]
    PlaceReference(PlaceReference),
    #[serde(rename = "referenceTransition")]
    TransitionReference(TransitionReference),
    #[serde(rename = "arc")]
    Arc(Arc),
    #[serde(rename = "name")]
    Name(Name),
    #[serde(rename = "toolspecific")]
    Toolspecific,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename = "net")]
pub struct Net<Place: Default, Transition: Default, Arc: Default> {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub name: Option<Name>,
    #[serde(rename = "page")]
    pub pages: Vec<Page<Place, Transition, Arc>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "pnml")]
pub struct Pnml<Net> {
    pub xmlns: String,
    #[serde(rename = "net")]
    pub nets: Vec<Net>,
}
