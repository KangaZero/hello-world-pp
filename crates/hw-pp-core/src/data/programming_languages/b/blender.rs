//! Hello-world snippet for `ProgrammingLanguageName::Blender`.
//!
//! Imported from leachim6/hello-world (`b/Blender.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Blender.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Blender => "Blender",
    versions: [Blender => "Blender"],
    extensions: [],
}

const BLENDER: &str = concat!(
    r#"import Blender
from Blender import Scene, Text3d

text = Text3d.New("Text")
text.setText(""#,
    output_for_hello_world!(),
    r#"")
Scene.GetCurrent().objects.new(text)
Blender.Redraw()"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Blender,
        extensions: &[],
        categories: &[Framework],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Blender,
            syntax: BLENDER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BLENDER,
                    [
                        (TokenKind::Keyword, r"import"),
                        (
                            TokenKind::Plain,
                            r" Blender
",
                        ),
                        (TokenKind::Keyword, r"from"),
                        (TokenKind::Plain, r" Blender "),
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Plain, r" Scene"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" Text3d

text ",
                        ),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" Text3d"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"New"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r#""Text""#),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
text",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"setText"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
Scene",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"GetCurrent"),
                        (TokenKind::Punctuation, r"()."),
                        (TokenKind::Plain, r"objects"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"new"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"text"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
Blender",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"Redraw"),
                        (TokenKind::Punctuation, r"()"),
                    ],
                )
            },
        }],
    };
