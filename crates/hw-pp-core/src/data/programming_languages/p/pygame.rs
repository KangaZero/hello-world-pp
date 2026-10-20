//! Hello-world snippet for `ProgrammingLanguageName::Pygame`.
//!
//! Imported from leachim6/hello-world (`p/Pygame.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pygame.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Python, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Pygame => "Pygame",
    versions: [Pygame => "Pygame"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pygame,
        extensions: &[],
        categories: &[Framework],
        families: &[Python],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pygame,
            syntax: concat!(
                r"import pygame, sys
from pygame.locals import QUIT

pygame.init()
display_surface = pygame.display.set_mode((400, 300))
font = pygame.font.Font(pygame.font.get_default_font(), 32)
text = font.render('",
                output_for_hello_world!(),
                r"', True, (0, 0, 0))
textRect = text.get_rect()

while True:
    display_surface.fill((255, 255, 255))
    display_surface.blit(text, textRect)
    for event in pygame.event.get():
        if event.type == QUIT:
            pygame.quit()
            sys.exit()

    pygame.display.update()"
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"import"),
                (TokenKind::Plain, r" pygame"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" sys
",
                ),
                (TokenKind::Keyword, r"from"),
                (TokenKind::Plain, r" pygame"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"locals "),
                (TokenKind::Keyword, r"import"),
                (
                    TokenKind::Plain,
                    r" QUIT

pygame",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"init"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r"
display_surface ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" pygame"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"display"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"set_mode"),
                (TokenKind::Punctuation, r"(("),
                (TokenKind::Number, r"400"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"300"),
                (TokenKind::Punctuation, r"))"),
                (
                    TokenKind::Plain,
                    r"
font ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" pygame"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"font"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"Font"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"pygame"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"font"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"get_default_font"),
                (TokenKind::Punctuation, r"(),"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"32"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
text ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" font"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"render"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r"'", output_for_hello_world!(), r"'"),
                ),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" True"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Number, r"0"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0"),
                (TokenKind::Punctuation, r"))"),
                (
                    TokenKind::Plain,
                    r"
textRect ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" text"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"get_rect"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"while"),
                (TokenKind::Plain, r" True"),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
    display_surface",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"fill"),
                (TokenKind::Punctuation, r"(("),
                (TokenKind::Number, r"255"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"255"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"255"),
                (TokenKind::Punctuation, r"))"),
                (
                    TokenKind::Plain,
                    r"
    display_surface",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"blit"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"text"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" textRect"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"for"),
                (TokenKind::Plain, r" event "),
                (TokenKind::Keyword, r"in"),
                (TokenKind::Plain, r" pygame"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"event"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"get"),
                (TokenKind::Punctuation, r"():"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"if"),
                (TokenKind::Plain, r" event"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"type "),
                (TokenKind::Keyword, r"=="),
                (TokenKind::Plain, r" QUIT"),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
            pygame",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"quit"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r"
            sys",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"exit"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r"

    pygame",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"display"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"update"),
                (TokenKind::Punctuation, r"()"),
            ],
        }],
    };
