//! Hello-world snippet for `ProgrammingLanguageName::Pangaea`.
//!
//! Imported from leachim6/hello-world (`p/Pangaea.pangaea` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pangaea.pangaea>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Pangaea => "Pangaea",
    versions: [Pangaea => "Pangaea"],
    extensions: [Pangaea => "pangaea"],
}

const PANGAEA: &str = concat!(r#"""#, output_for_hello_world!(), r#"".p"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pangaea,
        extensions: &[FileExtension::Pangaea],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pangaea,
            syntax: PANGAEA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PANGAEA,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#"".p"#),
                    )],
                )
            },
        }],
    };
