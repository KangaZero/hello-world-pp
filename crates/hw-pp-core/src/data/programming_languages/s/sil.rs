//! Hello-world snippet for `ProgrammingLanguageName::Sil`.
//!
//! Imported from leachim6/hello-world (`s/SIL.SIL` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SIL.SIL>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Sil => "SIL",
    versions: [Sil => "SIL"],
    extensions: [Sil => "SIL"],
}

const SIL: &str = concat!(r"print ", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sil,
        extensions: &[FileExtension::Sil],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sil,
            syntax: SIL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SIL,
                    [(
                        TokenKind::Plain,
                        concat!(r"print ", output_for_hello_world!()),
                    )],
                )
            },
        }],
    };
