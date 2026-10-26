//! Hello-world snippet for `ProgrammingLanguageName::Alaguf`.
//!
//! Imported from leachim6/hello-world (`a/ALAGUF.alaguf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ALAGUF.alaguf>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Alaguf => "ALAGUF",
    versions: [Alaguf => "ALAGUF"],
    extensions: [Alaguf => "alaguf"],
}

const ALAGUF: &str = concat!(r#"""#, output_for_hello_world!(), r##""#!"##);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Alaguf,
        extensions: &[FileExtension::Alaguf],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Alaguf,
            syntax: ALAGUF,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ALAGUF,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r##""#!"##),
                    )],
                )
            },
        }],
    };
