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
    programming_languages_token::TokenKind,
};

language_names! {
    language: Tex => "TeX",
    versions: [Tex => "TeX"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tex,
        extensions: &[],
        categories: &[Markup],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tex,
            syntax: concat!(
                output_for_hello_world!(),
                r"
\bye"
            ),
            tokenized_syntax: &[
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
        }],
    };
