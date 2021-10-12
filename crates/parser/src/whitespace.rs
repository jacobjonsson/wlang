use crate::Parser;

/// True if `c` is considered whitespace.
pub fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        '\u{0009}' // Tab
        | '\u{000B}' // Vertical tab
        | '\u{000C}' // Form feed
        | '\u{0020}' // Space
        | '\u{00A0}' // No-break space
        | '\u{FEFF}' // Zero width no-break space
        | '\u{1680}' // Orgham space mark
        | '\u{2000}' // En quad
        | '\u{2001}' // Em quad
        | '\u{2002}' // En space
        | '\u{2003}' // Em space
        | '\u{2004}' // Three per em space
        | '\u{2005}' // Four per em space
        | '\u{2006}' // Six per em space
        | '\u{2007}' // Figure space
        | '\u{2008}' // Punctuation space
        | '\u{2009}' // Thin space
        | '\u{200A}' // Hair space
        | '\u{202F}' // Narrow no-break space
        | '\u{205F}' // Medium mathematical space
        | '\u{3000}' // Ideographic space
    )
}

/// True if `c` is considered a line terminator.
pub fn is_line_terminator(c: char) -> bool {
    matches!(
        c,
        '\u{000A}' // Line feed
        | '\u{000D}' // Carriage return
        | '\u{2028}' // Line separator
        | '\u{2029}' // Paragraph separator
    )
}

impl<'a> Parser<'a> {
    pub(crate) fn skip_whitespace(&mut self) {
        while let Some(character) = self.current_character() {
            match character {
                c if is_whitespace(c) => {
                    self.bump();
                }

                c if is_line_terminator(c) => {
                    self.bump();
                }

                _ => break,
            }
        }
    }
}
