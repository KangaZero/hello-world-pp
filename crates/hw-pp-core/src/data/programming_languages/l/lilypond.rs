//! Hello-world snippet for `ProgrammingLanguageName::Lilypond`.
//!
//! Imported from leachim6/hello-world (`l/LilyPond.ly` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/LilyPond.ly>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Music,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Lilypond => "LilyPond",
    versions: [Lilypond => "LilyPond"],
    extensions: [Ly => "ly"],
}

const LILYPOND: &str = concat!(r"\markup { ", output_for_hello_world!(), r" }");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lilypond,
        extensions: &[FileExtension::Ly],
        categories: &[Music],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lilypond,
            syntax: LILYPOND,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LILYPOND,
                    [(
                        TokenKind::Plain,
                        concat!(r"\markup { ", output_for_hello_world!(), r" }"),
                    )],
                )
            },
        }],
    };
