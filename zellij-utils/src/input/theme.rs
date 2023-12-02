use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    collections::{BTreeMap, HashMap},
    fmt,
    hash::Hash,
};

use crate::{
    data::{PaletteColor, TermPalette, ThemeColorAssignments, ThemeHue},
    shared::colors,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct UiConfig {
    pub pane_frames: FrameConfig,
}

impl UiConfig {
    pub fn merge(&self, other: UiConfig) -> Self {
        let mut merged = self.clone();
        merged.pane_frames = merged.pane_frames.merge(other.pane_frames);
        merged
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct FrameConfig {
    pub rounded_corners: bool,
    pub hide_session_name: bool,
}

impl FrameConfig {
    pub fn merge(&self, other: FrameConfig) -> Self {
        let mut merged = self.clone();
        merged.rounded_corners = other.rounded_corners;
        merged.hide_session_name = other.hide_session_name;
        merged
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct Themes(HashMap<String, Theme>);

impl fmt::Debug for Themes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut stable_sorted = BTreeMap::new();
        for (theme_name, theme) in self.0.iter() {
            stable_sorted.insert(theme_name, theme);
        }
        write!(f, "{:#?}", stable_sorted)
    }
}

impl Themes {
    pub fn from_data(theme_data: HashMap<String, Theme>) -> Self {
        Themes(theme_data)
    }
    pub fn insert(&mut self, theme_name: String, theme: Theme) {
        self.0.insert(theme_name, theme);
    }
    pub fn merge(&self, mut other: Themes) -> Self {
        let mut merged = self.clone();
        for (name, theme) in other.0.drain() {
            merged.0.insert(name, theme);
        }
        merged
    }
    pub fn get_theme(&self, theme_name: &str) -> Option<&Theme> {
        self.0.get(theme_name)
    }
}

// TODO: move me to data
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Palette(BTreeMap<String, PaletteColor>);

/// Provide color defaults for backward compatiblity
impl Palette {
    pub fn new(colors: BTreeMap<String, PaletteColor>) -> Palette {
        let mut defaults = BTreeMap::new();
        defaults.insert("black".to_string(), PaletteColor::EightBit(colors::BLACK));
        defaults.insert("red".to_string(), PaletteColor::EightBit(colors::RED));
        defaults.insert("green".to_string(), PaletteColor::EightBit(colors::GREEN));
        defaults.insert("yellow".to_string(), PaletteColor::EightBit(colors::YELLOW));
        defaults.insert("blue".to_string(), PaletteColor::EightBit(colors::BLUE));
        defaults.insert(
            "magenta".to_string(),
            PaletteColor::EightBit(colors::MAGENTA),
        );
        defaults.insert("cyan".to_string(), PaletteColor::EightBit(colors::CYAN));
        defaults.insert("white".to_string(), PaletteColor::EightBit(colors::WHITE));
        defaults.insert("orange".to_string(), PaletteColor::EightBit(colors::ORANGE));
        defaults.insert("gray".to_string(), PaletteColor::EightBit(colors::GRAY));
        defaults.insert("gold".to_string(), PaletteColor::EightBit(colors::GOLD));
        defaults.insert("silver".to_string(), PaletteColor::EightBit(colors::SILVER));
        defaults.insert("pink".to_string(), PaletteColor::EightBit(colors::PINK));
        defaults.insert("brown".to_string(), PaletteColor::EightBit(colors::BROWN));
        defaults.extend(colors);
        Palette(defaults)
    }

    //TODO: should this be a read-only reference?
    pub fn get(&self, key: &str) -> Option<PaletteColor> {
        self.0.get(key).copied()
    }

    pub fn theme_hue(&self) -> ThemeHue {
        Default::default()
    }

    pub fn fg(&self) -> PaletteColor {
        let foreground = match self.theme_hue() {
            ThemeHue::Dark => &PaletteColor::EightBit(colors::WHITE),
            ThemeHue::Light => &PaletteColor::EightBit(colors::BLACK),
        };

        self.0.get("fg").unwrap_or(foreground).to_owned()
    }

    pub fn bg(&self) -> PaletteColor {
        let foreground = match self.theme_hue() {
            ThemeHue::Light => &PaletteColor::EightBit(colors::WHITE),
            ThemeHue::Dark => &PaletteColor::EightBit(colors::BLACK),
        };

        self.0.get("fg").unwrap_or(foreground).to_owned()
    }

    pub fn black(&self) -> PaletteColor {
        self.0
            .get("black")
            .unwrap_or(&PaletteColor::EightBit(colors::BLACK))
            .to_owned()
    }

    pub fn red(&self) -> PaletteColor {
        self.0
            .get("red")
            .unwrap_or(&PaletteColor::EightBit(colors::RED))
            .to_owned()
    }

    pub fn green(&self) -> PaletteColor {
        self.0
            .get("green")
            .unwrap_or(&PaletteColor::EightBit(colors::GREEN))
            .to_owned()
    }

    pub fn yellow(&self) -> PaletteColor {
        self.0
            .get("yellow")
            .unwrap_or(&PaletteColor::EightBit(colors::YELLOW))
            .to_owned()
    }

    pub fn blue(&self) -> PaletteColor {
        self.0
            .get("blue")
            .unwrap_or(&PaletteColor::EightBit(colors::BLUE))
            .to_owned()
    }
    pub fn magenta(&self) -> PaletteColor {
        self.0
            .get("magenta")
            .unwrap_or(&PaletteColor::EightBit(colors::MAGENTA))
            .to_owned()
    }
    pub fn cyan(&self) -> PaletteColor {
        self.0
            .get("cyan")
            .unwrap_or(&PaletteColor::EightBit(colors::CYAN))
            .to_owned()
    }
    pub fn white(&self) -> PaletteColor {
        self.0
            .get("white")
            .unwrap_or(&PaletteColor::EightBit(colors::WHITE))
            .to_owned()
    }
    pub fn orange(&self) -> PaletteColor {
        self.0
            .get("orange")
            .unwrap_or(&PaletteColor::EightBit(colors::ORANGE))
            .to_owned()
    }
    pub fn gray(&self) -> PaletteColor {
        self.0
            .get("gray")
            .unwrap_or(&PaletteColor::EightBit(colors::GRAY))
            .to_owned()
    }
    pub fn purple(&self) -> PaletteColor {
        self.0
            .get("purple")
            .unwrap_or(&PaletteColor::EightBit(colors::PURPLE))
            .to_owned()
    }
    pub fn gold(&self) -> PaletteColor {
        self.0
            .get("gold")
            .unwrap_or(&PaletteColor::EightBit(colors::GOLD))
            .to_owned()
    }
    pub fn silver(&self) -> PaletteColor {
        self.0
            .get("silver")
            .unwrap_or(&PaletteColor::EightBit(colors::SILVER))
            .to_owned()
    }
    pub fn pink(&self) -> PaletteColor {
        self.0
            .get("pink")
            .unwrap_or(&PaletteColor::EightBit(colors::PINK))
            .to_owned()
    }
    pub fn brown(&self) -> PaletteColor {
        self.0
            .get("brown")
            .unwrap_or(&PaletteColor::EightBit(colors::BROWN))
            .to_owned()
    }
}

impl Into<TermPalette> for Palette {
    fn into(self) -> TermPalette {
        TermPalette {
            fg: self.fg(),
            bg: self.bg(),
            black: self.black(),
            red: self.red(),
            green: self.green(),
            yellow: self.yellow(),
            blue: self.blue(),
            magenta: self.magenta(),
            cyan: self.cyan(),
            white: self.white(),
            orange: self.orange(),
            gray: self.gray(),
            purple: self.purple(),
            gold: self.gold(),
            silver: self.silver(),
            pink: self.pink(),
            brown: self.brown(),
            ..Default::default()
        }
    }
}

#[derive(Copy, Default, Hash, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub palette: TermPalette,
    pub styling: ThemeColorAssignments,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HexColor(u8, u8, u8);

impl From<HexColor> for (u8, u8, u8) {
    fn from(e: HexColor) -> (u8, u8, u8) {
        let HexColor(r, g, b) = e;
        (r, g, b)
    }
}

pub struct HexColorVisitor();

impl<'de> Visitor<'de> for HexColorVisitor {
    type Value = HexColor;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a hex color in the format #RGB or #RRGGBB")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if let Some(stripped) = s.strip_prefix('#') {
            return self.visit_str(stripped);
        }

        if s.len() == 3 {
            Ok(HexColor(
                u8::from_str_radix(&s[0..1], 16).map_err(E::custom)? * 0x11,
                u8::from_str_radix(&s[1..2], 16).map_err(E::custom)? * 0x11,
                u8::from_str_radix(&s[2..3], 16).map_err(E::custom)? * 0x11,
            ))
        } else if s.len() == 6 {
            Ok(HexColor(
                u8::from_str_radix(&s[0..2], 16).map_err(E::custom)?,
                u8::from_str_radix(&s[2..4], 16).map_err(E::custom)?,
                u8::from_str_radix(&s[4..6], 16).map_err(E::custom)?,
            ))
        } else {
            Err(Error::custom(
                "Hex color must be of form \"#RGB\" or \"#RRGGBB\"",
            ))
        }
    }
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(HexColorVisitor())
    }
}
impl Serialize for HexColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format!("{:02X}{:02X}{:02X}", self.0, self.1, self.2).as_str())
    }
}

#[cfg(test)]
#[path = "./unit/theme_test.rs"]
mod theme_test;
