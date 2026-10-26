//! Hello-world snippet for `ProgrammingLanguageName::Tex`.
//!
//! Imported from leachim6/hello-world (`t/TeX.tex` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/TeX.tex>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Markup, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Tex => "TeX",
    versions: [Tex => "TeX"],
    extensions: [],
}

const TEX: &str = concat!(
    output_for_hello_world!(),
    r"
\bye"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tex,
        extensions: &[],
        categories: &[Markup],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tex,
            syntax: TEX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TEX,
                    [
                        (
                            TokenKind::Plain,
                            concat!(
                                output_for_hello_world!(),
                                r"
"
                            ),
                        ),
                        (TokenKind::Punctuation, r"\"),
                        (TokenKind::Function, r"bye"),
                    ],
                )
            },
        }],
    };
