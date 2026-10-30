//! Hello-world snippet for `ProgrammingLanguageName::Perfume`.
//!
//! Imported from leachim6/hello-world (`p/Perfume.prfm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Perfume.prfm>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Perfume => "Perfume",
    versions: [Perfume => "Perfume"],
    extensions: [Prfm => "prfm"],
}

const PERFUME: &str = concat!(r#"println ""#, output_for_hello_world!(), r#"";"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Perfume,
        extensions: &[FileExtension::Prfm],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Perfume,
            syntax: PERFUME,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PERFUME,
                    [(
                        TokenKind::Plain,
                        concat!(r#"println ""#, output_for_hello_world!(), r#"";"#),
                    )],
                )
            },
        }],
    };
