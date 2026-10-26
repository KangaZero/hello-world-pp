//! Hello-world snippet for `ProgrammingLanguageName::Malbolge`.
//!
//! Imported from leachim6/hello-world (`m/Malbolge.mb` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Malbolge.mb>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Malbolge => "Malbolge",
    versions: [Malbolge => "Malbolge"],
    extensions: [Mb => "mb"],
}

const MALBOLGE: &str = r#"(=<`#9]~6ZY32Vx/4Rs+0No-&Jk)"Fh}|Bcy?,vNz]KZ%oG4UUS0/@-eMc(:'8"#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Malbolge,
        extensions: &[FileExtension::Mb],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Malbolge,
            syntax: MALBOLGE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MALBOLGE,
                    [(
                        TokenKind::Plain,
                        r#"(=<`#9]~6ZY32Vx/4Rs+0No-&Jk)"Fh}|Bcy?,vNz]KZ%oG4UUS0/@-eMc(:'8"#,
                    )],
                )
            },
        }],
    };
