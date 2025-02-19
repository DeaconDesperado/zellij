use std::{iter::Peekable, slice::IterMut, usize};

use super::{text::stringify_text, Coordinates, Text};
use crate::panes::terminal_character::{AnsiCode, CharacterStyles, RESET_STYLES};
use zellij_utils::data::{PaletteColor, Style};

static ARROW_SEPARATOR: &str = "";

pub fn ribbon(
    content: Text,
    style: &Style,
    arrow_fonts: bool,
    component_coordinates: Option<Coordinates>,
    index: Option<usize>,
) -> Vec<u8> {
    let colors = style.colors;
    let background = colors.text_unselected.background;

    let declaration = if content.selected {
        colors.ribbon_selected
    } else {
        colors.ribbon_unselected
    };

    let is_alternate_tab = index.map(|i| i % 2 == 0).unwrap_or_else(|| false);

    let (divider_character, padding, ribbon_bg) =
        match (arrow_fonts, content.selected, is_alternate_tab) {
            // Arrow fonts available, no alternating ribbon styling is applied
            (true, _, _) => (ARROW_SEPARATOR, Some(4), declaration.background),
            // Arrow fonts unavailable, alternate the background color if unselected
            (false, false, true) => ("", None, colors.ribbon_unselected.emphasis_1),
            (false, _, _) => ("", None, declaration.background),
        };

    let (first_arrow_styles, text_style, last_arrow_styles) = (
        character_style(background, ribbon_bg),
        character_style(declaration.base, ribbon_bg),
        character_style(ribbon_bg, background),
    );

    let (text, _text_width) = stringify_text(
        &content,
        padding,
        &component_coordinates,
        &declaration,
        text_style,
    );
    let mut stringified = component_coordinates
        .map(|c| c.to_string())
        .unwrap_or_else(|| String::new());
    stringified.push_str(&format!(
        "{}{}{}{} {} {}{}{}",
        RESET_STYLES,
        first_arrow_styles,
        divider_character,
        text_style,
        text,
        last_arrow_styles,
        divider_character,
        RESET_STYLES
    ));
    stringified.as_bytes().to_vec()
}

fn character_style(foreground: PaletteColor, background: PaletteColor) -> CharacterStyles {
    RESET_STYLES
        .foreground(Some(foreground.into()))
        .background(Some(background.into()))
        .bold(Some(AnsiCode::On))
}

pub(crate) fn parse_ribbon_index(params_iter: &mut Peekable<IterMut<'_, String>>) -> Option<usize> {
    params_iter
        .next_if(|frag| frag.starts_with('|') && frag.ends_with('|'))
        .and_then(|frag| {
            frag.strip_prefix('|')
                .and_then(|frag| frag.strip_suffix('|'))
                .and_then(|index_str| index_str.parse::<usize>().ok())
        })
}
