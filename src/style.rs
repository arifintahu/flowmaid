//! Shared color theme, exposed so hosts (GUI painters) can match
//! the SVG writers exactly.
//!
//! The default theme, [`Theme::Flowmaid`], is described below; the
//! others ([`Theme`]: mermaid's `default`, `neutral`, `dark`, `forest`)
//! are options, resolved into a [`Palette`] — every colour a writer
//! paints with — by [`crate::SvgOptions::palette`].
//!
//! Flowchart nodes are colored by shape semantics — the shape
//! already encodes meaning in Mermaid (stadium = start/end,
//! diamond = decision, ...), so color reinforces it for free.
//! ER entities cycle through an accent palette by index, which is
//! stable across renders because entity order is parse order.
//!
//! Text drawn on an accent fill picks its colour with [`text_on`]
//! (WCAG relative luminance), so header bands, wedges and branches stay
//! readable (≥ 4.5:1) whatever accent they land on.

use crate::model::Shape;
use std::borrow::Cow;

/// Fill + stroke pair, as `#rrggbb` hex strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShapeStyle {
    pub fill: &'static str,
    pub stroke: &'static str,
}

/// Semantic color per flowchart shape: pastel fill, saturated
/// stroke, all tuned to keep `#232840` text readable.
pub fn shape_style(shape: Shape) -> ShapeStyle {
    match shape {
        // Process step — the neutral workhorse, keeps the old indigo.
        Shape::Rect => ShapeStyle {
            fill: "#eef1fb",
            stroke: "#5b6dc0",
        },
        // Rounded step — teal, a softer sibling of Rect.
        Shape::Rounded => ShapeStyle {
            fill: "#e4f5f4",
            stroke: "#199a8e",
        },
        // Stadium (start / end) — green, "go".
        Shape::Stadium => ShapeStyle {
            fill: "#e5f5ea",
            stroke: "#33a35c",
        },
        // Diamond (decision) — amber, "attention".
        Shape::Diamond => ShapeStyle {
            fill: "#fcf2da",
            stroke: "#d99114",
        },
        // Circle (terminal / connector) — violet.
        Shape::Circle => ShapeStyle {
            fill: "#f2ecfa",
            stroke: "#8a5cd6",
        },
        // Double circle (terminal) — green like start/stop.
        Shape::DoubleCircle => ShapeStyle {
            fill: "#e5f5ea",
            stroke: "#33a35c",
        },
        // Cylinder (database / storage) — steel blue.
        Shape::Cylinder => ShapeStyle {
            fill: "#e7eef7",
            stroke: "#4d8fbf",
        },
        // Subroutine — indigo sibling of Rect.
        Shape::Subroutine => ShapeStyle {
            fill: "#eef1fb",
            stroke: "#5b6dc0",
        },
        // Hexagon (preparation) — violet.
        Shape::Hexagon => ShapeStyle {
            fill: "#f2ecfa",
            stroke: "#8a5cd6",
        },
        // Parallelograms (I/O) — amber.
        Shape::Parallelogram | Shape::ParallelogramAlt => ShapeStyle {
            fill: "#fcf2da",
            stroke: "#d99114",
        },
        // stateDiagram pseudostates (start/end dots, fork bar) —
        // solid dark, mermaid-style, not part of the shape theme.
        Shape::StateStart | Shape::StateEnd | Shape::ForkBar => ShapeStyle {
            fill: "#44507a",
            stroke: "#44507a",
        },
    }
}

/// Subgraph frame colours by nesting depth, outermost first (report Q6):
/// a cool grey a step deeper per level, so a nested frame reads apart
/// from the one round it, and a border at least 3:1 against what it is
/// drawn on — the white canvas, the parent frame's fill and its own —
/// where the old single `#c9cfe8` border managed 1.55:1 and every depth
/// looked the same. Greyer than the indigo node fill, so a node still
/// stands out of its frame; deliberately no hue per frame, which would
/// compete with the semantic node palette. Depths past the last share it
/// (see [`cluster_style`]).
pub const CLUSTER_STYLES: [ShapeStyle; 4] = [
    ShapeStyle {
        fill: "#f6f7fb",
        stroke: "#858db0",
    },
    ShapeStyle {
        fill: "#eceef6",
        stroke: "#7b83a6",
    },
    ShapeStyle {
        fill: "#e2e5f0",
        stroke: "#71799c",
    },
    ShapeStyle {
        fill: "#d8dcea",
        stroke: "#687093",
    },
];

/// Fill and border of a subgraph frame `depth` levels in (0 = outermost,
/// as `SceneCluster::depth` counts): [`CLUSTER_STYLES`], the deepest
/// entry for anything deeper.
pub fn cluster_style(depth: usize) -> ShapeStyle {
    CLUSTER_STYLES[depth.min(CLUSTER_STYLES.len() - 1)]
}

/// Ink of a subgraph frame's title: at least 5.7:1 on every
/// [`CLUSTER_STYLES`] fill (its chip takes the frame's fill), where the
/// old `#6a7086` fell under 4.5:1 on a nested frame.
pub const CLUSTER_TITLE: &str = "#4a5070";

/// Accent palette for ER entity headers (and anything else that
/// wants a stable per-item color). Mid-luminance by design: text on
/// these fills takes its colour from [`text_on`], never a fixed white.
pub const ACCENTS: &[&str] = &[
    "#5b6dc0", // indigo
    "#33a35c", // green
    "#d99114", // amber
    "#c2588c", // rose
    "#199a8e", // teal
    "#8a5cd6", // violet
    "#d96459", // coral
    "#4d8fbf", // steel blue
];

/// Stable accent for item `index` (wraps around the palette).
pub fn accent(index: usize) -> &'static str {
    ACCENTS[index % ACCENTS.len()]
}

/// [`ACCENTS`] darkened just enough (every sRGB channel scaled by the
/// same factor, so the hue holds) to reach [`MIN_TEXT_CONTRAST`]
/// against white — the colour for text drawn IN an item's accent on
/// the white canvas (gitgraph lane names and tags, sequence actor
/// labels). Entries that already pass (indigo, violet) are the accent
/// itself, so those keep their exact look. Index-parallel with
/// [`ACCENTS`].
pub const ACCENT_INKS: &[&str] = &[
    "#5b6dc0", // indigo (4.8:1 as is)
    "#2a864c", // green
    "#a16b0f", // amber
    "#b85485", // rose
    "#15847a", // teal
    "#8a5cd6", // violet (4.6:1 as is)
    "#be574e", // coral
    "#437ca5", // steel blue
];

/// [`ACCENT_INKS`] entry for item `index` — `accent(index)` made
/// readable as text on white (wraps like [`accent`]).
pub fn accent_ink(index: usize) -> &'static str {
    ACCENT_INKS[index % ACCENT_INKS.len()]
}

/// Second cycle of the accent palette: every [`ACCENTS`] entry with
/// its sRGB channels scaled by 0.65 (35% toward black). Deep enough to
/// stay distinct from the first cycle and from each other (CIE76
/// ΔE ≥ 20 across all sixteen), and white text clears 5.6:1 on each.
/// Used through [`accent_wide`]; index-parallel with [`ACCENTS`].
pub const ACCENTS_DEEP: &[&str] = &[
    "#3b477d", // deep indigo
    "#216a3c", // deep green
    "#8d5e0d", // deep amber
    "#7e395b", // deep rose
    "#10645c", // deep teal
    "#5a3c8b", // deep violet
    "#8d413a", // deep coral
    "#325d7c", // deep steel blue
];

/// Accent for item `index` from a sixteen-colour cycle — [`ACCENTS`],
/// then [`ACCENTS_DEEP`] — for renderers that can show more than
/// `ACCENTS.len()` items side by side (pie slices, sankey nodes,
/// journey actors), where [`accent`]'s wrap would give item 8 the
/// colour of item 0. Identical to [`accent`] for the first eight, so
/// switching a renderer over changes nothing for small diagrams. Pair
/// it with [`text_on`] for any text drawn on the fill.
pub fn accent_wide(index: usize) -> &'static str {
    let k = index % (ACCENTS.len() + ACCENTS_DEEP.len());
    if k < ACCENTS.len() {
        ACCENTS[k]
    } else {
        ACCENTS_DEEP[k - ACCENTS.len()]
    }
}

/// Note card colours (sequence notes and the like): a pale yellow card
/// with an olive-gold border, mermaid's note idiom. Deliberately
/// outside [`ACCENTS`] / [`ACCENTS_DEEP`] (border ΔE ≥ 20 from every
/// one of them) and away from the amber decision fill, so a note never
/// reads as a participant, a branch or a decision; the border keeps
/// 3:1 against the card and `#232840` text 13:1.
pub const NOTE_STYLE: ShapeStyle = ShapeStyle {
    fill: "#fff8c5",
    stroke: "#9c8a2c",
};

/// A state `note` card: the parser names it `__note_*`, draws it as a
/// [`Shape::Rect`](crate::model::Shape::Rect) and paints it in
/// [`NOTE_STYLE`] (a source state is only one when written exactly so).
pub(crate) fn is_note_card(
    id: &str,
    shape: crate::model::Shape,
    style: &crate::model::NodeStyle,
) -> bool {
    id.starts_with("__note_")
        && shape == crate::model::Shape::Rect
        && style.fill.as_deref() == Some(NOTE_STYLE.fill)
        && style.stroke.as_deref() == Some(NOTE_STYLE.stroke)
}

/// WCAG 2.x AA minimum contrast ratio for normal-size text.
pub const MIN_TEXT_CONTRAST: f64 = 4.5;

/// The dark choice of [`text_on`]: a navy near-black from the diagram
/// ink's family, deep enough that every [`ACCENTS`] entry white fails
/// on still reaches [`MIN_TEXT_CONTRAST`] — rose, the closest call,
/// gets 4.6:1 (a `#111827`-style ink only reaches 4.3:1 there).
pub const DARK_INK: &str = "#0a0e1a";

/// WCAG 2.x relative luminance (0 = black, 1 = white) of a `#rrggbb`
/// or `#rgb` colour; `None` for any other syntax (named colours,
/// `rgb()`, ...).
pub fn relative_luminance(color: &str) -> Option<f64> {
    let hex = color.strip_prefix('#')?;
    let d: Vec<u32> = hex.chars().map(|c| c.to_digit(16)).collect::<Option<_>>()?;
    let (r, g, b) = match d.len() {
        3 => (d[0] * 17, d[1] * 17, d[2] * 17),
        6 => (d[0] * 16 + d[1], d[2] * 16 + d[3], d[4] * 16 + d[5]),
        _ => return None,
    };
    // sRGB transfer curve → linear light.
    let lin = |c: u32| {
        let c = f64::from(c) / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    Some(0.2126 * lin(r) + 0.7152 * lin(g) + 0.0722 * lin(b))
}

/// WCAG 2.x contrast ratio (1 to 21) between two colours; `None` when
/// either is not `#rrggbb` / `#rgb`.
pub fn contrast_ratio(a: &str, b: &str) -> Option<f64> {
    let (la, lb) = (relative_luminance(a)?, relative_luminance(b)?);
    Some((la.max(lb) + 0.05) / (la.min(lb) + 0.05))
}

/// The text colour to draw ON a solid `fill` (accent header bands,
/// pie wedges, mindmap branches, ...): white wherever it reaches
/// [`MIN_TEXT_CONTRAST`], so fills that already read well keep their
/// look; otherwise [`DARK_INK`]; otherwise pure black, which always
/// clears 4.5:1 where white does not. A fill this cannot parse keeps
/// white, the historical default.
pub fn text_on(fill: &str) -> &'static str {
    const WHITE: &str = "#ffffff";
    let reads = |ink| contrast_ratio(fill, ink).map_or(true, |c| c >= MIN_TEXT_CONTRAST);
    if reads(WHITE) {
        WHITE
    } else if reads(DARK_INK) {
        DARK_INK
    } else {
        "#000000"
    }
}

// ------------------------------------------------------------------
// Themes
// ------------------------------------------------------------------

/// A colour theme for the SVG writers (report M10).
///
/// [`Theme::Flowmaid`] is the default and paints exactly what flowmaid
/// always has: nodes coloured by shape semantics, class / ER /
/// sequence items in a stable accent each. The others are options for
/// a mermaid-like look, every node alike: pick one with
/// [`crate::SvgOptions::theme`], `flowmaid --theme <name>`, or from the
/// diagram source (`%%{init: {"theme": "dark"}}%%`, or `theme:` under
/// a YAML frontmatter's `config:`).
///
/// Charts keep their categorical colours in every theme (pie slices,
/// git branches, journey actors, timeline sections, sankey nodes,
/// mindmap branches), as mermaid's do; their text, lines and canvas
/// follow the theme. `style`, `classDef` and `linkStyle` colours always
/// win over the theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum Theme {
    /// flowmaid's own semantic palette (the default).
    #[default]
    Flowmaid,
    /// mermaid's `default` theme: lavender `#ececff` nodes with a
    /// `#9370db` border, `#333333` lines and text, pale yellow frames.
    Mermaid,
    /// White nodes and one dark ink for borders, lines and text; grey
    /// frames (mermaid's `neutral`, with the white nodes of its newer
    /// look).
    Neutral,
    /// Light text, lines and borders on a near-black `#1e1f22` canvas
    /// (mermaid's `dark`); every text keeps 4.5:1 and every line 3:1.
    Dark,
    /// Green nodes with a deep green border, black lines (mermaid's
    /// `forest`).
    Forest,
}

impl Theme {
    /// Every theme, the default first.
    pub const ALL: [Theme; 5] = [
        Theme::Flowmaid,
        Theme::Mermaid,
        Theme::Neutral,
        Theme::Dark,
        Theme::Forest,
    ];

    /// The theme a name stands for, as mermaid spells them (`default`,
    /// `neutral`, `dark`, `forest`; any case, surrounding space
    /// ignored), plus `flowmaid` and `mermaid` (the same as `default`).
    /// mermaid's `base` (a theme meant to be recoloured through
    /// `themeVariables`) reads as `default`, its newer `neo` look as
    /// `neutral` and `neo-dark` as `dark`. `None` for any other name —
    /// callers fall back to the default, as mermaid does.
    pub fn from_name(name: &str) -> Option<Theme> {
        match name.trim().to_ascii_lowercase().as_str() {
            "flowmaid" => Some(Theme::Flowmaid),
            "default" | "mermaid" | "base" => Some(Theme::Mermaid),
            "neutral" | "neo" => Some(Theme::Neutral),
            "dark" | "neo-dark" => Some(Theme::Dark),
            "forest" => Some(Theme::Forest),
            _ => None,
        }
    }

    /// The theme's name: what [`Theme::from_name`] reads back to it.
    pub fn name(self) -> &'static str {
        match self {
            Theme::Flowmaid => "flowmaid",
            Theme::Mermaid => "default",
            Theme::Neutral => "neutral",
            Theme::Dark => "dark",
            Theme::Forest => "forest",
        }
    }

    /// The theme's colours ([`Palette::new`]).
    pub fn palette(self) -> Palette {
        Palette::new(self)
    }

    fn table(self) -> &'static ThemeTable {
        match self {
            Theme::Flowmaid => &FLOWMAID,
            Theme::Mermaid => &MERMAID,
            Theme::Neutral => &NEUTRAL,
            Theme::Dark => &DARK,
            Theme::Forest => &FOREST,
        }
    }
}

/// One theme's colours, as written in the source (see [`Palette`] for
/// what each role paints).
struct ThemeTable {
    background: &'static str,
    text: &'static str,
    muted_text: &'static str,
    line: &'static str,
    guide: &'static str,
    surface: &'static str,
    label_fill: &'static str,
    label_border: &'static str,
    /// Every node's fill and border; `None`: by shape ([`shape_style`]).
    node: Option<ShapeStyle>,
    node_text: &'static str,
    clusters: [ShapeStyle; 4],
    cluster_title: &'static str,
    note: ShapeStyle,
    note_text: &'static str,
    pseudo: &'static str,
}

/// [`Theme::Flowmaid`]: the colours every writer used before themes
/// existed, so the default output is unchanged byte for byte.
const FLOWMAID: ThemeTable = ThemeTable {
    background: "#ffffff",
    text: crate::scene::TEXT_COLOR,
    muted_text: crate::er::TYPE_COLOR,
    line: crate::scene::EDGE_COLOR,
    guide: "#aeb6d8",
    surface: "#ffffff",
    label_fill: "#ffffff",
    label_border: crate::scene::LABEL_BORDER,
    node: None,
    node_text: crate::scene::TEXT_COLOR,
    clusters: CLUSTER_STYLES,
    cluster_title: CLUSTER_TITLE,
    note: NOTE_STYLE,
    note_text: crate::scene::TEXT_COLOR,
    pseudo: crate::scene::EDGE_COLOR,
};

/// [`Theme::Mermaid`]: mermaid's `default` colours. Its frame and note
/// borders (`#aaaa33`, 2.4:1) are darkened, hue kept, to the 3:1 a
/// border needs on what it is drawn on, and the frames step a shade
/// deeper per nesting level as flowmaid's own do.
const MERMAID: ThemeTable = ThemeTable {
    background: "#ffffff",
    text: "#333333",
    muted_text: "#666666",
    line: "#333333",
    guide: "#888888",
    surface: "#ffffff",
    label_fill: "#e8e8e8",
    label_border: "#cccccc",
    node: Some(ShapeStyle {
        fill: "#ececff",
        stroke: "#9370db",
    }),
    node_text: "#333333",
    clusters: [
        ShapeStyle {
            fill: "#ffffde",
            stroke: "#97972d",
        },
        ShapeStyle {
            fill: "#f9f9cb",
            stroke: "#92922c",
        },
        ShapeStyle {
            fill: "#f3f3b8",
            stroke: "#8d8d2a",
        },
        ShapeStyle {
            fill: "#ededa5",
            stroke: "#8a8a29",
        },
    ],
    cluster_title: "#333333",
    note: ShapeStyle {
        fill: "#fff5ad",
        stroke: "#90902b",
    },
    note_text: "#333333",
    pseudo: "#333333",
};

/// [`Theme::Neutral`]: white nodes, one `#333333` ink.
const NEUTRAL: ThemeTable = ThemeTable {
    background: "#ffffff",
    text: "#333333",
    muted_text: "#666666",
    line: "#333333",
    guide: "#888888",
    surface: "#ffffff",
    label_fill: "#ffffff",
    label_border: "#cccccc",
    node: Some(ShapeStyle {
        fill: "#ffffff",
        stroke: "#333333",
    }),
    node_text: "#333333",
    clusters: [
        ShapeStyle {
            fill: "#f7f7f7",
            stroke: "#8a8a8a",
        },
        ShapeStyle {
            fill: "#eeeeee",
            stroke: "#898989",
        },
        ShapeStyle {
            fill: "#e5e5e5",
            stroke: "#838383",
        },
        ShapeStyle {
            fill: "#dcdcdc",
            stroke: "#7c7c7c",
        },
    ],
    cluster_title: "#333333",
    note: ShapeStyle {
        fill: "#f2f2f2",
        stroke: "#666666",
    },
    note_text: "#333333",
    pseudo: "#333333",
};

/// [`Theme::Dark`]: mermaid's `dark` idea — light text, lines and
/// borders on near-black — on a deeper canvas than its `#333333`, so
/// every [`ACCENTS`] line (git lanes, mindmap branches) keeps 3:1 on it,
/// with nodes and frames a step lighter than the canvas. Where mermaid
/// falls short on a dark page (its frame border is a 25% white, its
/// edge-label text 4.4:1 on the chip) every border and line here keeps
/// 3:1 and every text 4.5:1.
const DARK: ThemeTable = ThemeTable {
    background: "#1e1f22",
    text: "#e6e6e6",
    muted_text: "#b4b4b4",
    line: "#d0d0d0",
    guide: "#8a8a8a",
    surface: "#2b2d31",
    label_fill: "#2b2d31",
    label_border: "#5a5d63",
    node: Some(ShapeStyle {
        fill: "#2b2d31",
        stroke: "#c8c8c8",
    }),
    node_text: "#e6e6e6",
    clusters: [
        ShapeStyle {
            fill: "#26282c",
            stroke: "#7a7d82",
        },
        ShapeStyle {
            fill: "#2e3035",
            stroke: "#7a7d82",
        },
        ShapeStyle {
            fill: "#36393e",
            stroke: "#7f8287",
        },
        ShapeStyle {
            fill: "#3e4147",
            stroke: "#898b90",
        },
    ],
    cluster_title: "#f0f0f0",
    note: ShapeStyle {
        fill: "#3d3a26",
        stroke: "#a69641",
    },
    note_text: "#f2f2f2",
    pseudo: "#f0f0f0",
};

/// [`Theme::Forest`]: mermaid's `forest` greens; frame and note borders
/// darkened to 3:1 as in [`Theme::Mermaid`].
const FOREST: ThemeTable = ThemeTable {
    background: "#ffffff",
    text: "#000000",
    muted_text: "#4d4d4d",
    line: "#000000",
    guide: "#6f8f62",
    surface: "#ffffff",
    label_fill: "#e8e8e8",
    label_border: "#cccccc",
    node: Some(ShapeStyle {
        fill: "#cde498",
        stroke: "#13540c",
    }),
    node_text: "#000000",
    clusters: [
        ShapeStyle {
            fill: "#cdffb2",
            stroke: "#639942",
        },
        ShapeStyle {
            fill: "#bdf3a0",
            stroke: "#5c8f3d",
        },
        ShapeStyle {
            fill: "#aee78f",
            stroke: "#568539",
        },
        ShapeStyle {
            fill: "#9fdb7e",
            stroke: "#507c35",
        },
    ],
    cluster_title: "#000000",
    note: ShapeStyle {
        fill: "#fff5ad",
        stroke: "#639942",
    },
    note_text: "#000000",
    pseudo: "#000000",
};

/// mermaid `themeVariables` a diagram's source may set, as sRGB
/// channels: Copy, so [`crate::SvgOptions`] can carry them. Only the
/// handful that recolour the roles every writer shares are read.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct ThemeVars {
    pub(crate) background: Option<[u8; 3]>,
    pub(crate) primary: Option<[u8; 3]>,
    pub(crate) primary_border: Option<[u8; 3]>,
    pub(crate) primary_text: Option<[u8; 3]>,
    pub(crate) line: Option<[u8; 3]>,
    pub(crate) text: Option<[u8; 3]>,
}

impl ThemeVars {
    /// Take one `themeVariables` entry: `background`, `primaryColor`,
    /// `primaryBorderColor`, `primaryTextColor`, `lineColor` or
    /// `textColor`, any colour [`css_rgb`] reads. Other keys, and
    /// values that are no colour, are ignored (theme config is
    /// decoration, never a reason to fail).
    pub(crate) fn set(&mut self, key: &str, value: &str) {
        let slot = match key {
            "background" => &mut self.background,
            "primaryColor" => &mut self.primary,
            "primaryBorderColor" => &mut self.primary_border,
            "primaryTextColor" => &mut self.primary_text,
            "lineColor" => &mut self.line,
            "textColor" => &mut self.text,
            _ => return,
        };
        if let Some(rgb) = css_rgb(value) {
            *slot = Some(rgb);
        }
    }
}

fn hex([r, g, b]: [u8; 3]) -> String {
    format!("#{r:02x}{g:02x}{b:02x}")
}

/// Every colour a theme paints with, resolved: the shared roles the
/// SVG writers draw from, so a host painting a [`crate::scene::Scene`]
/// can match the SVG exactly (`opts.palette()` for the options it
/// renders with, see [`crate::SvgOptions::palette`]). All colours are
/// `#rrggbb`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Palette {
    /// The theme the colours come from.
    pub theme: Theme,
    /// The canvas (the background `<rect>`).
    pub background: String,
    /// Text on the canvas and on light boxes: edge labels, member rows,
    /// messages, captions.
    pub text: String,
    /// Secondary text: ER attribute types.
    pub muted_text: String,
    /// Edges, relationship lines, messages and their end glyphs.
    pub line: String,
    /// Faint structural lines: lifelines, frames, axes, rails.
    pub guide: String,
    /// The fill of plain boxes on the canvas: class and ER bodies,
    /// activation bars, architecture services, hollow end glyphs.
    pub surface: String,
    /// The chip behind an edge or relationship label.
    pub label_fill: String,
    /// The chip's border.
    pub label_border: String,
    /// Text inside a node (and a class / ER / sequence header when the
    /// theme paints items alike, see [`Palette::node_style`]).
    pub node_text: String,
    /// A subgraph / composite title.
    pub cluster_title: String,
    /// A note card's fill.
    pub note_fill: String,
    /// A note card's border.
    pub note_stroke: String,
    /// A note card's text.
    pub note_text: String,
    /// State start / end dots and fork bars.
    pub pseudo: String,
    node_fill: Option<String>,
    node_stroke: Option<String>,
    clusters: Vec<(String, String)>,
}

impl Palette {
    /// `theme`'s colours.
    pub fn new(theme: Theme) -> Palette {
        Palette::with_vars(theme, &ThemeVars::default())
    }

    /// `theme`'s colours with the source's `themeVariables` laid over
    /// them. A `primaryColor` without a `primaryTextColor` keeps the
    /// theme's node text where it still reads (4.5:1), else takes
    /// [`text_on`] the new fill.
    pub(crate) fn with_vars(theme: Theme, vars: &ThemeVars) -> Palette {
        let t = theme.table();
        let pick = |v: Option<[u8; 3]>, base: &str| v.map_or_else(|| base.to_string(), hex);
        let node_fill = vars.primary.map(hex).or(t.node.map(|n| n.fill.to_string()));
        let node_stroke = vars
            .primary_border
            .map(hex)
            .or(t.node.map(|n| n.stroke.to_string()));
        let node_text = match (vars.primary_text, vars.primary) {
            (Some(c), _) => hex(c),
            (None, Some(fill)) => {
                let fill = hex(fill);
                let reads =
                    contrast_ratio(t.node_text, &fill).is_some_and(|c| c >= MIN_TEXT_CONTRAST);
                if reads {
                    t.node_text.to_string()
                } else {
                    text_on(&fill).to_string()
                }
            }
            (None, None) => t.node_text.to_string(),
        };
        Palette {
            theme,
            background: pick(vars.background, t.background),
            text: pick(vars.text, t.text),
            muted_text: t.muted_text.to_string(),
            line: pick(vars.line, t.line),
            guide: t.guide.to_string(),
            surface: t.surface.to_string(),
            label_fill: t.label_fill.to_string(),
            label_border: t.label_border.to_string(),
            node_text,
            cluster_title: t.cluster_title.to_string(),
            note_fill: t.note.fill.to_string(),
            note_stroke: t.note.stroke.to_string(),
            note_text: t.note_text.to_string(),
            pseudo: t.pseudo.to_string(),
            node_fill,
            node_stroke,
            clusters: t
                .clusters
                .iter()
                .map(|c| (c.fill.to_string(), c.stroke.to_string()))
                .collect(),
        }
    }

    /// Fill and border of an unstyled node of `shape`: the theme's one
    /// node paint, or, in [`Theme::Flowmaid`], [`shape_style`]'s colour
    /// for the shape. State start / end dots and fork bars are
    /// [`Palette::pseudo`] in every theme.
    pub fn node_style(&self, shape: Shape) -> (&str, &str) {
        if matches!(shape, Shape::StateStart | Shape::StateEnd | Shape::ForkBar) {
            return (&self.pseudo, &self.pseudo);
        }
        let by_shape = shape_style(shape);
        (
            self.node_fill.as_deref().unwrap_or(by_shape.fill),
            self.node_stroke.as_deref().unwrap_or(by_shape.stroke),
        )
    }

    /// Fill and border of a subgraph frame `depth` levels in (0 =
    /// outermost), a step deeper per level like [`cluster_style`].
    pub fn cluster_style(&self, depth: usize) -> (&str, &str) {
        let (f, s) = &self.clusters[depth.min(self.clusters.len() - 1)];
        (f, s)
    }

    /// How item `index` of a class / ER / sequence diagram is painted:
    /// its own [`accent`] (text on it by [`text_on`], on the canvas in
    /// [`accent_ink`]) while nodes are coloured by shape, else the
    /// theme's node paint for every item, as mermaid paints them.
    pub(crate) fn item(&self, index: usize) -> ItemPaint<'_> {
        match &self.node_fill {
            None => {
                let a = accent(index);
                ItemPaint {
                    fill: a,
                    stroke: a,
                    ink: text_on(a),
                    canvas_ink: Cow::Owned(self.accent_ink(index)),
                }
            }
            Some(fill) => ItemPaint {
                fill,
                stroke: self.node_stroke.as_deref().unwrap_or(&self.line),
                ink: &self.node_text,
                canvas_ink: Cow::Borrowed(&self.text),
            },
        }
    }

    /// [`accent_ink`] for item `index` made readable on this canvas:
    /// the accent darkened for white as it is, or, where that falls
    /// under 4.5:1 (a dark canvas), the accent lightened until it reads
    /// ([`readable_on`]).
    pub fn accent_ink(&self, index: usize) -> String {
        let ink = accent_ink(index);
        if contrast_ratio(ink, &self.background).is_some_and(|c| c >= MIN_TEXT_CONTRAST) {
            ink.to_string()
        } else {
            readable_on(accent(index), &self.background)
        }
    }

    /// The text ink for a node whose fill the author may have set
    /// (`style` / `classDef` without a `color`): the theme's `ink`, or,
    /// outside [`Theme::Flowmaid`], [`text_on`] that fill where the
    /// theme's ink does not read on it (4.5:1) — a theme changes the
    /// ink under fills the author picked for flowmaid's dark one (a
    /// pale highlight under [`Theme::Dark`]'s light text). The default
    /// theme keeps its ink, so its output is unchanged.
    pub(crate) fn ink_on_fill<'a>(&self, fill: Option<&str>, ink: &'a str) -> &'a str {
        if self.theme == Theme::Flowmaid {
            return ink;
        }
        match fill.and_then(css_hex) {
            Some(fill) if contrast_ratio(ink, &fill).is_some_and(|c| c < MIN_TEXT_CONTRAST) => {
                text_on(&fill)
            }
            _ => ink,
        }
    }

    /// `flowmaid` — one writer's own tuned constant for a role (the
    /// sequence lifeline grey, the timeline axis, ...) — in
    /// [`Theme::Flowmaid`], so the default output keeps every one of
    /// them exactly; `token`, the theme's colour for the role, in any
    /// other theme.
    pub(crate) fn tuned<'a>(&'a self, flowmaid: &'a str, token: &'a str) -> &'a str {
        if self.theme == Theme::Flowmaid {
            flowmaid
        } else {
            token
        }
    }
}

/// [`Palette::item`]: one class box's, ER entity's or sequence
/// participant's colours.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ItemPaint<'a> {
    /// The header band / box fill.
    pub(crate) fill: &'a str,
    /// Its border (and dividers).
    pub(crate) stroke: &'a str,
    /// Text on `fill`.
    pub(crate) ink: &'a str,
    /// Text in the item's colour straight on the canvas (an actor's
    /// name under its stick figure), readable there.
    pub(crate) canvas_ink: Cow<'a, str>,
}

/// `color` as `#rrggbb`, moved toward white (on a dark `bg`) or black
/// (on a light one) just far enough to reach [`MIN_TEXT_CONTRAST`]
/// against `bg`; unchanged where it already does. A colour that is not
/// `#rgb`, `#rrggbb`, a CSS name or `rgb()` comes back as it is.
pub fn readable_on(color: &str, bg: &str) -> String {
    let (Some(rgb), Some(bg_hex)) = (css_rgb(color), css_hex(bg)) else {
        return color.to_string();
    };
    let reads = |c: &str| contrast_ratio(c, &bg_hex).is_some_and(|k| k >= MIN_TEXT_CONTRAST);
    let start = hex(rgb);
    if reads(&start) {
        return start;
    }
    let to_white = relative_luminance(&bg_hex).is_some_and(|l| l < 0.18);
    // 64 even steps: enough to land within a few units of the least
    // move that reads, and the far end (white / black) always does on
    // a background past the 0.18 split.
    for k in 1..=64 {
        let t = f64::from(k) / 64.0;
        let mixed = rgb.map(|c| {
            let c = f64::from(c);
            let target = if to_white { 255.0 } else { 0.0 };
            (c + (target - c) * t).round() as u8
        });
        let h = hex(mixed);
        if reads(&h) {
            return h;
        }
    }
    text_on(&bg_hex).to_string()
}

/// CSS Color 4's named colours (lower case, sorted for a binary search).
const CSS_NAMED: [(&str, u32); 148] = [
    ("aliceblue", 0xf0f8ff),
    ("antiquewhite", 0xfaebd7),
    ("aqua", 0x00ffff),
    ("aquamarine", 0x7fffd4),
    ("azure", 0xf0ffff),
    ("beige", 0xf5f5dc),
    ("bisque", 0xffe4c4),
    ("black", 0x000000),
    ("blanchedalmond", 0xffebcd),
    ("blue", 0x0000ff),
    ("blueviolet", 0x8a2be2),
    ("brown", 0xa52a2a),
    ("burlywood", 0xdeb887),
    ("cadetblue", 0x5f9ea0),
    ("chartreuse", 0x7fff00),
    ("chocolate", 0xd2691e),
    ("coral", 0xff7f50),
    ("cornflowerblue", 0x6495ed),
    ("cornsilk", 0xfff8dc),
    ("crimson", 0xdc143c),
    ("cyan", 0x00ffff),
    ("darkblue", 0x00008b),
    ("darkcyan", 0x008b8b),
    ("darkgoldenrod", 0xb8860b),
    ("darkgray", 0xa9a9a9),
    ("darkgreen", 0x006400),
    ("darkgrey", 0xa9a9a9),
    ("darkkhaki", 0xbdb76b),
    ("darkmagenta", 0x8b008b),
    ("darkolivegreen", 0x556b2f),
    ("darkorange", 0xff8c00),
    ("darkorchid", 0x9932cc),
    ("darkred", 0x8b0000),
    ("darksalmon", 0xe9967a),
    ("darkseagreen", 0x8fbc8f),
    ("darkslateblue", 0x483d8b),
    ("darkslategray", 0x2f4f4f),
    ("darkslategrey", 0x2f4f4f),
    ("darkturquoise", 0x00ced1),
    ("darkviolet", 0x9400d3),
    ("deeppink", 0xff1493),
    ("deepskyblue", 0x00bfff),
    ("dimgray", 0x696969),
    ("dimgrey", 0x696969),
    ("dodgerblue", 0x1e90ff),
    ("firebrick", 0xb22222),
    ("floralwhite", 0xfffaf0),
    ("forestgreen", 0x228b22),
    ("fuchsia", 0xff00ff),
    ("gainsboro", 0xdcdcdc),
    ("ghostwhite", 0xf8f8ff),
    ("gold", 0xffd700),
    ("goldenrod", 0xdaa520),
    ("gray", 0x808080),
    ("green", 0x008000),
    ("greenyellow", 0xadff2f),
    ("grey", 0x808080),
    ("honeydew", 0xf0fff0),
    ("hotpink", 0xff69b4),
    ("indianred", 0xcd5c5c),
    ("indigo", 0x4b0082),
    ("ivory", 0xfffff0),
    ("khaki", 0xf0e68c),
    ("lavender", 0xe6e6fa),
    ("lavenderblush", 0xfff0f5),
    ("lawngreen", 0x7cfc00),
    ("lemonchiffon", 0xfffacd),
    ("lightblue", 0xadd8e6),
    ("lightcoral", 0xf08080),
    ("lightcyan", 0xe0ffff),
    ("lightgoldenrodyellow", 0xfafad2),
    ("lightgray", 0xd3d3d3),
    ("lightgreen", 0x90ee90),
    ("lightgrey", 0xd3d3d3),
    ("lightpink", 0xffb6c1),
    ("lightsalmon", 0xffa07a),
    ("lightseagreen", 0x20b2aa),
    ("lightskyblue", 0x87cefa),
    ("lightslategray", 0x778899),
    ("lightslategrey", 0x778899),
    ("lightsteelblue", 0xb0c4de),
    ("lightyellow", 0xffffe0),
    ("lime", 0x00ff00),
    ("limegreen", 0x32cd32),
    ("linen", 0xfaf0e6),
    ("magenta", 0xff00ff),
    ("maroon", 0x800000),
    ("mediumaquamarine", 0x66cdaa),
    ("mediumblue", 0x0000cd),
    ("mediumorchid", 0xba55d3),
    ("mediumpurple", 0x9370db),
    ("mediumseagreen", 0x3cb371),
    ("mediumslateblue", 0x7b68ee),
    ("mediumspringgreen", 0x00fa9a),
    ("mediumturquoise", 0x48d1cc),
    ("mediumvioletred", 0xc71585),
    ("midnightblue", 0x191970),
    ("mintcream", 0xf5fffa),
    ("mistyrose", 0xffe4e1),
    ("moccasin", 0xffe4b5),
    ("navajowhite", 0xffdead),
    ("navy", 0x000080),
    ("oldlace", 0xfdf5e6),
    ("olive", 0x808000),
    ("olivedrab", 0x6b8e23),
    ("orange", 0xffa500),
    ("orangered", 0xff4500),
    ("orchid", 0xda70d6),
    ("palegoldenrod", 0xeee8aa),
    ("palegreen", 0x98fb98),
    ("paleturquoise", 0xafeeee),
    ("palevioletred", 0xdb7093),
    ("papayawhip", 0xffefd5),
    ("peachpuff", 0xffdab9),
    ("peru", 0xcd853f),
    ("pink", 0xffc0cb),
    ("plum", 0xdda0dd),
    ("powderblue", 0xb0e0e6),
    ("purple", 0x800080),
    ("rebeccapurple", 0x663399),
    ("red", 0xff0000),
    ("rosybrown", 0xbc8f8f),
    ("royalblue", 0x4169e1),
    ("saddlebrown", 0x8b4513),
    ("salmon", 0xfa8072),
    ("sandybrown", 0xf4a460),
    ("seagreen", 0x2e8b57),
    ("seashell", 0xfff5ee),
    ("sienna", 0xa0522d),
    ("silver", 0xc0c0c0),
    ("skyblue", 0x87ceeb),
    ("slateblue", 0x6a5acd),
    ("slategray", 0x708090),
    ("slategrey", 0x708090),
    ("snow", 0xfffafa),
    ("springgreen", 0x00ff7f),
    ("steelblue", 0x4682b4),
    ("tan", 0xd2b48c),
    ("teal", 0x008080),
    ("thistle", 0xd8bfd8),
    ("tomato", 0xff6347),
    ("turquoise", 0x40e0d0),
    ("violet", 0xee82ee),
    ("wheat", 0xf5deb3),
    ("white", 0xffffff),
    ("whitesmoke", 0xf5f5f5),
    ("yellow", 0xffff00),
    ("yellowgreen", 0x9acd32),
];

/// The sRGB channels of a CSS colour a user may write in a diagram's
/// config or `classDef`: `#rgb`, `#rrggbb` (with or without alpha
/// digits, which are ignored), a CSS named colour (any case), or
/// `rgb()` / `rgba()` with numbers or percentages. `None` for anything
/// else (`transparent`, `hsl()`, `var(...)`, ...).
pub(crate) fn css_rgb(color: &str) -> Option<[u8; 3]> {
    let c = color.trim();
    if let Some(hex) = c.strip_prefix('#') {
        let d: Vec<u8> = hex
            .chars()
            .map(|ch| ch.to_digit(16).map(|v| v as u8))
            .collect::<Option<_>>()?;
        return match d.len() {
            3 | 4 => Some([d[0] * 17, d[1] * 17, d[2] * 17]),
            6 | 8 => Some([d[0] * 16 + d[1], d[2] * 16 + d[3], d[4] * 16 + d[5]]),
            _ => None,
        };
    }
    let lower = c.to_ascii_lowercase();
    if let Some(args) = lower
        .strip_prefix("rgba(")
        .or_else(|| lower.strip_prefix("rgb("))
        .and_then(|r| r.strip_suffix(')'))
    {
        let parts: Vec<&str> = args
            .split(|ch: char| ch == ',' || ch == '/' || ch.is_whitespace())
            .filter(|p| !p.is_empty())
            .collect();
        if parts.len() < 3 {
            return None;
        }
        let channel = |p: &str| -> Option<u8> {
            let v = match p.strip_suffix('%') {
                Some(pct) => pct.parse::<f64>().ok()? * 255.0 / 100.0,
                None => p.parse::<f64>().ok()?,
            };
            v.is_finite().then(|| v.round().clamp(0.0, 255.0) as u8)
        };
        return Some([channel(parts[0])?, channel(parts[1])?, channel(parts[2])?]);
    }
    let at = CSS_NAMED
        .binary_search_by(|(name, _)| (*name).cmp(lower.as_str()))
        .ok()?;
    let v = CSS_NAMED[at].1;
    Some([(v >> 16) as u8, (v >> 8) as u8, v as u8])
}

/// `color` as `#rrggbb` when [`css_rgb`] reads it (so a named colour
/// can be measured by [`contrast_ratio`]), else `None`.
pub(crate) fn css_hex(color: &str) -> Option<String> {
    css_rgb(color).map(|[r, g, b]| format!("#{r:02x}{g:02x}{b:02x}"))
}

/// Test helper for the renderers' contrast regressions: asserts the
/// first `<text>` whose content (or a `<tspan>` line of it) is exactly
/// `label` is painted in an ink reaching [`MIN_TEXT_CONTRAST`] on
/// `bg`, and returns that ink.
#[cfg(test)]
pub(crate) fn assert_text_reads<'a>(svg: &'a str, label: &str, bg: &str) -> &'a str {
    let end = [format!(">{label}</text>"), format!(">{label}</tspan>")]
        .iter()
        .filter_map(|pat| svg.find(pat.as_str()))
        .min()
        .unwrap_or_else(|| panic!("no <text> reading {label:?} in\n{svg}"));
    let open = svg[..end].rfind("<text").expect("enclosing <text>");
    let tag = &svg[open..end];
    let f = tag.find(" fill=\"").expect("<text> has a fill") + 7;
    let ink = &tag[f..f + tag[f..].find('"').unwrap()];
    let c = contrast_ratio(ink, bg).expect("hex colours");
    assert!(
        c >= MIN_TEXT_CONTRAST,
        "{label:?}: {ink} on {bg} is only {c:.2}:1"
    );
    ink
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn css_colours_read_as_rgb() {
        assert_eq!(css_rgb("#4d8fbf"), Some([0x4d, 0x8f, 0xbf]));
        assert_eq!(css_rgb("#abc"), Some([0xaa, 0xbb, 0xcc]));
        assert_eq!(css_rgb("#4d8fbf80"), Some([0x4d, 0x8f, 0xbf]));
        assert_eq!(css_rgb("SteelBlue"), Some([0x46, 0x82, 0xb4]));
        assert_eq!(css_rgb(" rebeccapurple "), Some([0x66, 0x33, 0x99]));
        assert_eq!(css_rgb("rgb(255, 0, 128)"), Some([255, 0, 128]));
        assert_eq!(css_rgb("rgba(100%,50%,0%,0.5)"), Some([255, 128, 0]));
        assert_eq!(css_rgb("rgb(0 0 0 / 50%)"), Some([0, 0, 0]));
        for bad in [
            "transparent",
            "hsl(0,0%,0%)",
            "#12",
            "notacolour",
            "rgb(1,2)",
            "",
        ] {
            assert_eq!(css_rgb(bad), None, "{bad}");
        }
        assert_eq!(css_hex("yellow").as_deref(), Some("#ffff00"));
        // The table stays sorted, or the binary search misses names.
        assert!(CSS_NAMED.windows(2).all(|w| w[0].0 < w[1].0));
    }

    #[test]
    fn every_shape_has_a_distinct_stroke() {
        let shapes = [
            Shape::Rect,
            Shape::Rounded,
            Shape::Stadium,
            Shape::Diamond,
            Shape::Circle,
        ];
        for (i, a) in shapes.iter().enumerate() {
            for b in &shapes[i + 1..] {
                assert_ne!(shape_style(*a).stroke, shape_style(*b).stroke);
            }
        }
    }

    #[test]
    fn accent_wraps_stably() {
        assert_eq!(accent(0), accent(ACCENTS.len()));
        assert_ne!(accent(0), accent(1));
    }

    fn rgb(c: &str) -> [f64; 3] {
        let h = c.strip_prefix('#').unwrap();
        let ch = |i: usize| f64::from(u8::from_str_radix(&h[i..i + 2], 16).unwrap());
        [ch(0), ch(2), ch(4)]
    }

    /// CIE76 ΔE between two `#rrggbb` colours (D65), for the
    /// distinctness checks below.
    fn delta_e(a: &str, b: &str) -> f64 {
        let lab = |c: &str| {
            let [r, g, b] = rgb(c).map(|v| {
                let v = v / 255.0;
                if v <= 0.04045 {
                    v / 12.92
                } else {
                    ((v + 0.055) / 1.055).powf(2.4)
                }
            });
            let x = (0.4124 * r + 0.3576 * g + 0.1805 * b) / 0.95047;
            let y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
            let z = (0.0193 * r + 0.1192 * g + 0.9505 * b) / 1.08883;
            let f = |t: f64| {
                if t > 216.0 / 24389.0 {
                    t.cbrt()
                } else {
                    (24389.0 / 27.0 * t + 16.0) / 116.0
                }
            };
            let (fx, fy, fz) = (f(x), f(y), f(z));
            [116.0 * fy - 16.0, 500.0 * (fx - fy), 200.0 * (fy - fz)]
        };
        let (p, q) = (lab(a), lab(b));
        ((p[0] - q[0]).powi(2) + (p[1] - q[1]).powi(2) + (p[2] - q[2]).powi(2)).sqrt()
    }

    /// `c` is `base` with every channel scaled by one factor `k <= 1`
    /// (within the ±1 of hex rounding) — same hue, only darker.
    fn is_shade_of(c: &str, base: &str) -> bool {
        let (c, b) = (rgb(c), rgb(base));
        let k = c[1] / b[1];
        k <= 1.0 + 1e-9 && (0..3).all(|i| (c[i] - b[i] * k).abs() <= 1.5)
    }

    #[test]
    fn cluster_frames_step_by_depth_with_borders_that_read() {
        // Report Q6: every border reaches 3:1 on what it is drawn on (the
        // canvas, its parent frame's fill, its own fill), each depth's
        // fill differs from its parent's, and the title stays AA on all.
        let mut parent = "#ffffff";
        for (d, c) in CLUSTER_STYLES.iter().enumerate() {
            for on in ["#ffffff", parent, c.fill] {
                let k = contrast_ratio(c.stroke, on).unwrap();
                assert!(k >= 3.0, "depth {d}: border {} on {on} {k:.2}", c.stroke);
            }
            let step = contrast_ratio(c.fill, parent).unwrap();
            assert!(
                step >= 1.05,
                "depth {d}: fill too close to its parent's ({step:.3})"
            );
            let t = contrast_ratio(CLUSTER_TITLE, c.fill).unwrap();
            assert!(t >= MIN_TEXT_CONTRAST, "depth {d}: title {t:.2}");
            assert_eq!(cluster_style(d), *c);
            parent = c.fill;
        }
        assert_eq!(cluster_style(99), CLUSTER_STYLES[3]);
    }

    #[test]
    fn luminance_matches_wcag_reference_values() {
        let near = |a: Option<f64>, b: f64| (a.unwrap() - b).abs() < 1e-3;
        assert!(near(relative_luminance("#ffffff"), 1.0));
        assert!(near(relative_luminance("#000000"), 0.0));
        assert!(near(relative_luminance("#fff"), 1.0), "#rgb shorthand");
        assert!(
            near(relative_luminance("#FF0000"), 0.2126),
            "case-insensitive"
        );
        assert!(near(contrast_ratio("#000", "#fff"), 21.0));
        assert!(near(contrast_ratio("#777777", "#ffffff"), 4.478));
        // Symmetric.
        assert_eq!(
            contrast_ratio("#5b6dc0", "#ffffff"),
            contrast_ratio("#ffffff", "#5b6dc0")
        );
        for bad in ["red", "#12345", "#gggggg", "5b6dc0", "", "#ééé"] {
            assert_eq!(relative_luminance(bad), None, "{bad}");
        }
    }

    /// Report Q5 / §2.11: white text hard-coded on every accent failed
    /// WCAG AA on six of the eight (amber 2.6:1, green 3.2:1, ...).
    #[test]
    fn every_accent_gets_text_reaching_aa() {
        for &a in ACCENTS.iter().chain(ACCENTS_DEEP) {
            let ink = text_on(a);
            let c = contrast_ratio(a, ink).unwrap();
            assert!(c >= MIN_TEXT_CONTRAST, "{ink} on {a}: {c:.2}:1");
            // Always one of the two designed inks, never the black
            // last resort, so the palette reads as one system.
            assert!(ink == "#ffffff" || ink == DARK_INK, "{a} → {ink}");
        }
    }

    #[test]
    fn text_on_keeps_white_where_it_already_passes() {
        // Indigo (4.8:1) and violet (4.6:1) keep their white headers.
        assert_eq!(text_on("#5b6dc0"), "#ffffff");
        assert_eq!(text_on("#8a5cd6"), "#ffffff");
        // Amber (2.6:1 with white) flips to the dark ink.
        assert_eq!(text_on("#d99114"), DARK_INK);
        assert_eq!(text_on("#ffffff"), DARK_INK);
        // Unparseable fills keep the historical white.
        assert_eq!(text_on("tomato"), "#ffffff");
    }

    #[test]
    fn text_on_reaches_aa_on_any_fill() {
        // Sweep the whole grey ramp plus saturated primaries: whatever
        // the fill, the chosen ink clears 4.5:1 — including the band
        // (luminance ~0.18-0.22) where neither white nor DARK_INK does
        // and pure black takes over.
        let mut fills: Vec<String> = (0..=255)
            .map(|v| format!("#{v:02x}{v:02x}{v:02x}"))
            .collect();
        for c in [
            "#ff0000", "#00ff00", "#0000ff", "#ffff00", "#00ffff", "#ff00ff",
        ] {
            fills.push(c.to_string());
        }
        for f in &fills {
            let c = contrast_ratio(f, text_on(f)).unwrap();
            assert!(c >= MIN_TEXT_CONTRAST, "{f}: {c:.2}:1");
        }
    }

    #[test]
    fn accent_inks_read_on_white_and_keep_their_hue() {
        assert_eq!(ACCENT_INKS.len(), ACCENTS.len());
        for (i, (&ink, &a)) in ACCENT_INKS.iter().zip(ACCENTS).enumerate() {
            let c = contrast_ratio(ink, "#ffffff").unwrap();
            assert!(c >= MIN_TEXT_CONTRAST, "ink {i} {ink}: {c:.2}:1");
            assert!(is_shade_of(ink, a), "ink {ink} is not a shade of {a}");
            // Accents that already pass are not touched at all.
            if contrast_ratio(a, "#ffffff").unwrap() >= MIN_TEXT_CONTRAST {
                assert_eq!(ink, a);
            }
            assert_eq!(accent_ink(i), ink);
            assert_eq!(accent_ink(i + ACCENTS.len()), ink, "wraps like accent()");
        }
    }

    #[test]
    fn wide_accent_cycle_is_distinct_and_extends_accent() {
        assert_eq!(ACCENTS_DEEP.len(), ACCENTS.len());
        let n = ACCENTS.len();
        // The first cycle is exactly `accent`, so adopting it changes
        // nothing for diagrams with eight items or fewer.
        for i in 0..n {
            assert_eq!(accent_wide(i), accent(i));
            assert!(is_shade_of(accent_wide(i + n), accent(i)));
        }
        assert_eq!(accent_wide(0), accent_wide(2 * n), "wraps after 16");
        // Sixteen genuinely different colours: item 8 no longer
        // repeats item 0 (pie slice 9, journey actor 9, sankey).
        let all: Vec<&str> = (0..2 * n).map(accent_wide).collect();
        for (i, a) in all.iter().enumerate() {
            for b in &all[i + 1..] {
                let d = delta_e(a, b);
                assert!(d >= 20.0, "{a} vs {b}: ΔE {d:.1}");
            }
        }
    }

    #[test]
    fn theme_names_read_back_and_unknown_ones_are_none() {
        for t in Theme::ALL {
            assert_eq!(Theme::from_name(t.name()), Some(t));
        }
        assert_eq!(Theme::default(), Theme::Flowmaid);
        assert_eq!(Theme::from_name(" Dark "), Some(Theme::Dark));
        assert_eq!(Theme::from_name("MERMAID"), Some(Theme::Mermaid));
        assert_eq!(Theme::from_name("base"), Some(Theme::Mermaid));
        assert_eq!(Theme::from_name("neo"), Some(Theme::Neutral));
        assert_eq!(Theme::from_name("neo-dark"), Some(Theme::Dark));
        for bad in ["", "redux", "darkest", "default-dark"] {
            assert_eq!(Theme::from_name(bad), None, "{bad}");
        }
    }

    /// The default theme's palette is exactly the colours every writer
    /// used before themes existed — the root of the default output
    /// staying byte-identical.
    #[test]
    fn flowmaid_palette_is_the_historical_colours() {
        let p = Palette::new(Theme::Flowmaid);
        assert_eq!(p.background, "#ffffff");
        assert_eq!(p.text, crate::scene::TEXT_COLOR);
        assert_eq!(p.line, crate::scene::EDGE_COLOR);
        assert_eq!(p.label_border, crate::scene::LABEL_BORDER);
        assert_eq!(p.muted_text, crate::er::TYPE_COLOR);
        assert_eq!(
            (p.surface.as_str(), p.label_fill.as_str()),
            ("#ffffff", "#ffffff")
        );
        assert_eq!(p.cluster_title, CLUSTER_TITLE);
        assert_eq!(
            (p.note_fill.as_str(), p.note_stroke.as_str()),
            (NOTE_STYLE.fill, NOTE_STYLE.stroke)
        );
        for shape in ALL_SHAPES {
            let ss = shape_style(shape);
            assert_eq!(p.node_style(shape), (ss.fill, ss.stroke), "{shape:?}");
        }
        for d in 0..6 {
            let c = cluster_style(d);
            assert_eq!(p.cluster_style(d), (c.fill, c.stroke));
        }
        for i in 0..20 {
            let it = p.item(i);
            assert_eq!(
                (it.fill, it.stroke, it.ink),
                (accent(i), accent(i), text_on(accent(i)))
            );
            assert_eq!(it.canvas_ink, accent_ink(i));
            assert_eq!(p.accent_ink(i), accent_ink(i));
        }
        assert_eq!(p.tuned("#123456", "#abcdef"), "#123456");
        assert_eq!(
            Palette::new(Theme::Dark).tuned("#123456", "#abcdef"),
            "#abcdef"
        );
    }

    const ALL_SHAPES: [Shape; 15] = [
        Shape::Rect,
        Shape::Rounded,
        Shape::Stadium,
        Shape::Diamond,
        Shape::Circle,
        Shape::DoubleCircle,
        Shape::Cylinder,
        Shape::Subroutine,
        Shape::Hexagon,
        Shape::Parallelogram,
        Shape::ParallelogramAlt,
        Shape::StateStart,
        Shape::StateEnd,
        Shape::ForkBar,
        Shape::Rect,
    ];

    /// Report M10 / §2.11: every theme's text reaches 4.5:1 on whatever
    /// it is drawn on, and its lines and borders 3:1 — the dark theme
    /// above all, where the default colours would vanish.
    #[test]
    fn every_theme_keeps_text_and_lines_readable() {
        let ok = |a: &str, b: &str, min: f64, what: &str| {
            let c = contrast_ratio(a, b).unwrap();
            assert!(c >= min, "{what}: {a} on {b} is {c:.2}:1");
        };
        for t in Theme::ALL {
            let p = Palette::new(t);
            let name = t.name();
            for (ink, what) in [(&p.text, "text"), (&p.muted_text, "muted text")] {
                for bg in [&p.background, &p.surface] {
                    ok(ink, bg, MIN_TEXT_CONTRAST, &format!("{name} {what}"));
                }
            }
            ok(
                &p.text,
                &p.label_fill,
                MIN_TEXT_CONTRAST,
                &format!("{name} label"),
            );
            ok(
                &p.note_text,
                &p.note_fill,
                MIN_TEXT_CONTRAST,
                &format!("{name} note"),
            );
            ok(
                &p.note_stroke,
                &p.note_fill,
                3.0,
                &format!("{name} note border"),
            );
            ok(&p.line, &p.background, 3.0, &format!("{name} line"));
            ok(&p.line, &p.surface, 3.0, &format!("{name} line on surface"));
            ok(
                &p.pseudo,
                &p.background,
                3.0,
                &format!("{name} pseudostate"),
            );
            // ER keys are drawn in the line colour, on the table body.
            ok(
                &p.line,
                &p.surface,
                MIN_TEXT_CONTRAST,
                &format!("{name} keys"),
            );
            for shape in ALL_SHAPES {
                let (fill, stroke) = p.node_style(shape);
                if matches!(shape, Shape::StateStart | Shape::StateEnd | Shape::ForkBar) {
                    continue;
                }
                ok(
                    &p.node_text,
                    fill,
                    MIN_TEXT_CONTRAST,
                    &format!("{name} {shape:?} text"),
                );
                if t != Theme::Flowmaid {
                    ok(
                        stroke,
                        &p.background,
                        3.0,
                        &format!("{name} {shape:?} border"),
                    );
                }
            }
            let mut parent = p.background.clone();
            for d in 0..4 {
                let (fill, stroke) = p.cluster_style(d);
                for on in [p.background.as_str(), parent.as_str(), fill] {
                    ok(stroke, on, 3.0, &format!("{name} frame {d} border"));
                }
                ok(
                    &p.cluster_title,
                    fill,
                    MIN_TEXT_CONTRAST,
                    &format!("{name} frame {d} title"),
                );
                parent = fill.to_string();
            }
            for i in 0..16 {
                let it = p.item(i);
                ok(
                    it.ink,
                    it.fill,
                    MIN_TEXT_CONTRAST,
                    &format!("{name} item {i}"),
                );
                ok(
                    &it.canvas_ink,
                    &p.background,
                    MIN_TEXT_CONTRAST,
                    &format!("{name} item ink {i}"),
                );
                ok(
                    &p.accent_ink(i),
                    &p.background,
                    MIN_TEXT_CONTRAST,
                    &format!("{name} accent ink {i}"),
                );
            }
            if t == Theme::Flowmaid {
                continue; // its tuned guides are faint by design
            }
            ok(&p.guide, &p.background, 3.0, &format!("{name} guide"));
            // Lines drawn in an accent (git lanes, mindmap branches) keep
            // 3:1 on the dark canvas, chosen for it; on white they are
            // the default theme's palette as it always was.
            if t == Theme::Dark {
                for &a in ACCENTS {
                    ok(a, &p.background, 3.0, &format!("{name} accent line"));
                }
            }
        }
    }

    #[test]
    fn themed_palettes_paint_nodes_alike() {
        for t in [Theme::Mermaid, Theme::Neutral, Theme::Dark, Theme::Forest] {
            let p = Palette::new(t);
            let rect = p.node_style(Shape::Rect);
            for shape in ALL_SHAPES {
                if !matches!(shape, Shape::StateStart | Shape::StateEnd | Shape::ForkBar) {
                    assert_eq!(p.node_style(shape), rect, "{t:?} {shape:?}");
                }
            }
            assert_eq!(p.item(0), p.item(7), "{t:?}: items alike");
            assert_eq!(p.item(0).fill, rect.0);
        }
        // M10's yardstick: neutral is white nodes in one ink.
        let n = Palette::new(Theme::Neutral);
        assert_eq!(n.node_style(Shape::Diamond), ("#ffffff", "#333333"));
        assert_eq!((n.text.as_str(), n.line.as_str()), ("#333333", "#333333"));
        let m = Palette::new(Theme::Mermaid);
        assert_eq!(m.node_style(Shape::Stadium), ("#ececff", "#9370db"));
    }

    #[test]
    fn theme_variables_recolour_the_shared_roles() {
        let mut v = ThemeVars::default();
        v.set("primaryColor", "#1a237e");
        v.set("lineColor", "red");
        v.set("background", "rgb(250, 250, 250)");
        v.set("textColor", "not a colour");
        v.set("someOtherKey", "#ffffff");
        let p = Palette::with_vars(Theme::Mermaid, &v);
        assert_eq!(p.node_style(Shape::Rect), ("#1a237e", "#9370db"));
        assert_eq!(p.line, "#ff0000");
        assert_eq!(p.background, "#fafafa");
        assert_eq!(p.text, "#333333", "a value that is no colour is ignored");
        // The theme's dark node text no longer reads on the deep fill.
        assert_eq!(p.node_text, "#ffffff");
        v.set("primaryTextColor", "#ffeb3b");
        v.set("primaryBorderColor", "#000");
        let p = Palette::with_vars(Theme::Mermaid, &v);
        assert_eq!(p.node_text, "#ffeb3b");
        assert_eq!(p.node_style(Shape::Diamond).1, "#000000");
        // In the default theme a fill alone keeps each shape's border.
        let mut v = ThemeVars::default();
        v.set("primaryColor", "#fafafa");
        let p = Palette::with_vars(Theme::Flowmaid, &v);
        assert_eq!(
            p.node_style(Shape::Diamond),
            ("#fafafa", shape_style(Shape::Diamond).stroke)
        );
        assert_eq!(p.node_text, crate::scene::TEXT_COLOR, "still reads");
    }

    #[test]
    fn readable_on_moves_a_colour_just_far_enough() {
        // Already readable: unchanged (normalised to #rrggbb).
        assert_eq!(readable_on("#5B6DC0", "#ffffff"), "#5b6dc0");
        // Too dark for a dark canvas: lightened, hue kept roughly.
        let dark = Palette::new(Theme::Dark).background;
        for &a in ACCENTS.iter().chain(ACCENTS_DEEP) {
            let r = readable_on(a, &dark);
            let c = contrast_ratio(&r, &dark).unwrap();
            assert!(c >= MIN_TEXT_CONTRAST, "{a} -> {r}: {c:.2}");
            // Lighter, never past white.
            assert!(relative_luminance(&r).unwrap() >= relative_luminance(a).unwrap());
        }
        // Too light for white: darkened.
        let r = readable_on("#ffeb3b", "#ffffff");
        assert!(
            contrast_ratio(&r, "#ffffff").unwrap() >= MIN_TEXT_CONTRAST,
            "{r}"
        );
        assert_eq!(readable_on("var(--x)", "#000"), "var(--x)");
    }

    #[test]
    fn a_theme_keeps_text_readable_on_an_authors_fill() {
        let dark = Palette::new(Theme::Dark);
        // A pale highlight picked for flowmaid's dark ink.
        assert_eq!(dark.ink_on_fill(Some("#fef3c7"), &dark.node_text), DARK_INK);
        // A fill the theme's ink reads on, no fill, or no colour: kept.
        assert_eq!(dark.ink_on_fill(Some("#1e293b"), "#e6e6e6"), "#e6e6e6");
        assert_eq!(dark.ink_on_fill(None, "#e6e6e6"), "#e6e6e6");
        assert_eq!(dark.ink_on_fill(Some("transparent"), "#e6e6e6"), "#e6e6e6");
        // The default theme never second-guesses an author.
        let fm = Palette::new(Theme::Flowmaid);
        assert_eq!(fm.ink_on_fill(Some("#000000"), "#232840"), "#232840");
    }

    #[test]
    fn note_style_stands_apart_from_the_accents() {
        let ShapeStyle { fill, stroke } = NOTE_STYLE;
        for &a in ACCENTS.iter().chain(ACCENTS_DEEP) {
            assert!(delta_e(stroke, a) >= 20.0, "note border vs {a}");
        }
        // Not the amber decision / attention fill either.
        let diamond = shape_style(Shape::Diamond);
        assert!(delta_e(fill, diamond.fill) >= 10.0);
        assert!(delta_e(stroke, diamond.stroke) >= 20.0);
        assert!(
            contrast_ratio(stroke, fill).unwrap() >= 3.0,
            "border visible"
        );
        let text = contrast_ratio(crate::scene::TEXT_COLOR, fill).unwrap();
        assert!(text >= MIN_TEXT_CONTRAST);
    }
}
