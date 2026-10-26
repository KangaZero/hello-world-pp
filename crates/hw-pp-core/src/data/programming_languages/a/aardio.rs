//! Hello-world snippet for `ProgrammingLanguageName::Aardio`.
//!
//! Imported from leachim6/hello-world (`a/aardio.aardio` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/aardio.aardio>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Aardio => "aardio",
    versions: [Aardio => "aardio"],
    extensions: [Aardio => "aardio"],
}

const AARDIO: &str = concat!(
    r#"import console
console.log(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Aardio,
        extensions: &[FileExtension::Aardio],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Aardio,
            syntax: AARDIO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AARDIO,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"import console
console.log(""#,
                            output_for_hello_world!(),
                            r#"")"#
                        ),
                    )],
                )
            },
        }],
    };
