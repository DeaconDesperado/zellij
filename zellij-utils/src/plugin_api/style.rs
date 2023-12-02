use super::generated_api::api::style::{
    color::Payload as ProtobufColorPayload, Color as ProtobufColor, ColorType as ProtobufColorType,
    Palette as ProtobufPalette, RgbColorPayload as ProtobufRgbColorPayload, Style as ProtobufStyle,
    StyleSpec as ProtobufStyleSpec, ThemeColorAssignments as ProtobufThemeColorAssignments,
    ThemeHue as ProtobufThemeHue,
};
use crate::data::{PaletteColor, Style, StyleSpec, TermPalette, ThemeColorAssignments, ThemeHue};
use crate::errors::prelude::*;
use crate::input::theme::Theme;

use std::convert::TryFrom;

impl TryFrom<ProtobufStyle> for Style {
    type Error = &'static str;
    fn try_from(protobuf_style: ProtobufStyle) -> Result<Self, &'static str> {
        let palette = protobuf_style.palette.ok_or("Invalid palette")?;
        let styling = protobuf_style.styling.ok_or("Invalid style")?;

        Ok(Style {
            theme: Theme {
                palette: TermPalette::try_from(palette)?,
                styling: ThemeColorAssignments::try_from(styling)?,
            },
            rounded_corners: protobuf_style.rounded_corners,
            hide_session_name: protobuf_style.hide_session_name,
        })
    }
}

impl TryFrom<Style> for ProtobufStyle {
    type Error = &'static str;
    fn try_from(style: Style) -> Result<Self, &'static str> {
        let palette = ProtobufPalette::try_from(style.theme.palette)?;
        let styling = ProtobufThemeColorAssignments::try_from(style.theme.styling)?;

        Ok(ProtobufStyle {
            palette: Some(palette),
            rounded_corners: style.rounded_corners,
            hide_session_name: style.hide_session_name,
            styling: Some(styling),
        })
    }
}

impl TryFrom<ProtobufPalette> for TermPalette {
    type Error = &'static str;
    fn try_from(protobuf_palette: ProtobufPalette) -> Result<Self, &'static str> {
        Ok(TermPalette {
            theme_hue: ProtobufThemeHue::from_i32(protobuf_palette.theme_hue)
                .ok_or("malformed theme_hue payload for Palette")?
                .try_into()?,
            fg: protobuf_palette
                .fg
                .ok_or("malformed palette payload")?
                .try_into()?,
            bg: protobuf_palette
                .bg
                .ok_or("malformed palette payload")?
                .try_into()?,
            black: protobuf_palette
                .black
                .ok_or("malformed palette payload")?
                .try_into()?,
            red: protobuf_palette
                .red
                .ok_or("malformed palette payload")?
                .try_into()?,
            green: protobuf_palette
                .green
                .ok_or("malformed palette payload")?
                .try_into()?,
            yellow: protobuf_palette
                .yellow
                .ok_or("malformed palette payload")?
                .try_into()?,
            blue: protobuf_palette
                .blue
                .ok_or("malformed palette payload")?
                .try_into()?,
            magenta: protobuf_palette
                .magenta
                .ok_or("malformed palette payload")?
                .try_into()?,
            cyan: protobuf_palette
                .cyan
                .ok_or("malformed palette payload")?
                .try_into()?,
            white: protobuf_palette
                .white
                .ok_or("malformed palette payload")?
                .try_into()?,
            orange: protobuf_palette
                .orange
                .ok_or("malformed palette payload")?
                .try_into()?,
            gray: protobuf_palette
                .gray
                .ok_or("malformed palette payload")?
                .try_into()?,
            purple: protobuf_palette
                .purple
                .ok_or("malformed palette payload")?
                .try_into()?,
            gold: protobuf_palette
                .gold
                .ok_or("malformed palette payload")?
                .try_into()?,
            silver: protobuf_palette
                .silver
                .ok_or("malformed palette payload")?
                .try_into()?,
            pink: protobuf_palette
                .pink
                .ok_or("malformed palette payload")?
                .try_into()?,
            brown: protobuf_palette
                .brown
                .ok_or("malformed palette payload")?
                .try_into()?,
            ..Default::default()
        })
    }
}

impl TryFrom<TermPalette> for ProtobufPalette {
    type Error = &'static str;
    fn try_from(palette: TermPalette) -> Result<Self, &'static str> {
        let theme_hue: ProtobufThemeHue = palette
            .theme_hue
            .try_into()
            .map_err(|_| "malformed payload for palette")?;
        Ok(ProtobufPalette {
            theme_hue: theme_hue as i32,
            fg: Some(palette.fg.try_into()?),
            bg: Some(palette.bg.try_into()?),
            black: Some(palette.black.try_into()?),
            red: Some(palette.red.try_into()?),
            green: Some(palette.green.try_into()?),
            yellow: Some(palette.yellow.try_into()?),
            blue: Some(palette.blue.try_into()?),
            magenta: Some(palette.magenta.try_into()?),
            cyan: Some(palette.cyan.try_into()?),
            white: Some(palette.white.try_into()?),
            orange: Some(palette.orange.try_into()?),
            gray: Some(palette.gray.try_into()?),
            purple: Some(palette.purple.try_into()?),
            gold: Some(palette.gold.try_into()?),
            silver: Some(palette.silver.try_into()?),
            pink: Some(palette.pink.try_into()?),
            brown: Some(palette.brown.try_into()?),
            ..Default::default()
        })
    }
}

impl TryFrom<ProtobufColor> for PaletteColor {
    type Error = &'static str;
    fn try_from(protobuf_color: ProtobufColor) -> Result<Self, &'static str> {
        match ProtobufColorType::from_i32(protobuf_color.color_type) {
            Some(ProtobufColorType::Rgb) => match protobuf_color.payload {
                Some(ProtobufColorPayload::RgbColorPayload(rgb_color_payload)) => {
                    Ok(PaletteColor::Rgb((
                        rgb_color_payload.red as u8,
                        rgb_color_payload.green as u8,
                        rgb_color_payload.blue as u8,
                    )))
                },
                _ => Err("malformed payload for Rgb color"),
            },
            Some(ProtobufColorType::EightBit) => match protobuf_color.payload {
                Some(ProtobufColorPayload::EightBitColorPayload(eight_bit_payload)) => {
                    Ok(PaletteColor::EightBit(eight_bit_payload as u8))
                },
                _ => Err("malformed payload for 8bit color"),
            },
            None => Err("malformed payload for Color"),
        }
    }
}

impl TryFrom<PaletteColor> for ProtobufColor {
    type Error = &'static str;
    fn try_from(color: PaletteColor) -> Result<Self, &'static str> {
        match color {
            PaletteColor::Rgb((red, green, blue)) => {
                let red = red as u32;
                let green = green as u32;
                let blue = blue as u32;
                Ok(ProtobufColor {
                    color_type: ProtobufColorType::Rgb as i32,
                    payload: Some(ProtobufColorPayload::RgbColorPayload(
                        ProtobufRgbColorPayload { red, green, blue },
                    )),
                })
            },
            PaletteColor::EightBit(color) => Ok(ProtobufColor {
                color_type: ProtobufColorType::EightBit as i32,
                payload: Some(ProtobufColorPayload::EightBitColorPayload(color as u32)),
            }),
        }
    }
}

impl TryFrom<ThemeHue> for ProtobufThemeHue {
    type Error = &'static str;
    fn try_from(theme_hue: ThemeHue) -> Result<Self, &'static str> {
        match theme_hue {
            ThemeHue::Light => Ok(ProtobufThemeHue::Light),
            ThemeHue::Dark => Ok(ProtobufThemeHue::Dark),
        }
    }
}

impl TryFrom<ProtobufThemeHue> for ThemeHue {
    type Error = &'static str;
    fn try_from(protobuf_theme_hue: ProtobufThemeHue) -> Result<Self, &'static str> {
        match protobuf_theme_hue {
            ProtobufThemeHue::Light => Ok(ThemeHue::Light),
            ProtobufThemeHue::Dark => Ok(ThemeHue::Dark),
        }
    }
}

impl TryFrom<ProtobufThemeColorAssignments> for ThemeColorAssignments {
    type Error = &'static str;

    fn try_from(value: ProtobufThemeColorAssignments) -> std::result::Result<Self, Self::Error> {
        let selected_ribbon = value
            .selected_ribbon
            .ok_or("missing")
            .and_then(|v| StyleSpec::try_from(v))?;

        let unselected_ribbon = value
            .unselected_ribbon
            .ok_or("missing")
            .and_then(|v| StyleSpec::try_from(v))?;

        let key = value
            .key
            .ok_or("missing")
            .and_then(|v| StyleSpec::try_from(v))?;

        let key_modifier = value
            .key_modifier
            .ok_or("missing")
            .and_then(|v| StyleSpec::try_from(v))?;

        let text = value
            .text
            .ok_or("missing")
            .and_then(|v| StyleSpec::try_from(v))?;

        let error_text = value
            .error_text
            .ok_or("missing")
            .and_then(|v| StyleSpec::try_from(v))?;

        let selected_frame = value
            .selected_frame
            .ok_or("missing")
            .and_then(|v| StyleSpec::try_from(v))?;

        Ok(ThemeColorAssignments {
            selected_ribbon,
            unselected_ribbon,
            key,
            key_modifier,
            selected_frame,
            text,
            error_text,
        })
    }
}

impl TryFrom<ThemeColorAssignments> for ProtobufThemeColorAssignments {
    type Error = &'static str;

    fn try_from(value: ThemeColorAssignments) -> std::result::Result<Self, Self::Error> {
        Ok(ProtobufThemeColorAssignments {
            selected_ribbon: Some(ProtobufStyleSpec {
                fg: Some(value.selected_ribbon.fg.try_into()?),
                bg: Some(value.selected_ribbon.bg.try_into()?),
            }),
            unselected_ribbon: Some(ProtobufStyleSpec {
                fg: Some(value.unselected_ribbon.fg.try_into()?),
                bg: Some(value.unselected_ribbon.bg.try_into()?),
            }),
            text: Some(ProtobufStyleSpec {
                fg: Some(value.text.fg.try_into()?),
                bg: Some(value.text.bg.try_into()?),
            }),
            key: Some(ProtobufStyleSpec {
                fg: Some(value.key.fg.try_into()?),
                bg: Some(value.key.bg.try_into()?),
            }),
            key_modifier: Some(ProtobufStyleSpec {
                fg: Some(value.key_modifier.fg.try_into()?),
                bg: Some(value.key_modifier.bg.try_into()?),
            }),
            error_text: Some(ProtobufStyleSpec {
                fg: Some(value.error_text.fg.try_into()?),
                bg: Some(value.error_text.bg.try_into()?),
            }),
            selected_frame: Some(ProtobufStyleSpec {
                fg: Some(value.selected_frame.fg.try_into()?),
                bg: Some(value.selected_frame.bg.try_into()?),
            }),
        })
    }
}

impl TryFrom<ProtobufStyleSpec> for StyleSpec {
    type Error = &'static str;

    fn try_from(value: ProtobufStyleSpec) -> std::result::Result<Self, Self::Error> {
        let fg = value
            .fg
            .ok_or("foreground color missing")
            .and_then(|f| PaletteColor::try_from(f))?;

        let bg = value
            .bg
            .ok_or("background color missing")
            .and_then(|f| PaletteColor::try_from(f))?;

        Ok(StyleSpec { fg, bg })
    }
}

impl TryFrom<StyleSpec> for ProtobufStyleSpec {
    type Error = &'static str;

    fn try_from(value: StyleSpec) -> std::result::Result<Self, Self::Error> {
        let fg = ProtobufColor::try_from(value.fg)?;
        let bg = ProtobufColor::try_from(value.bg)?;
        Ok(ProtobufStyleSpec {
            fg: Some(fg),
            bg: Some(bg),
        })
    }
}
