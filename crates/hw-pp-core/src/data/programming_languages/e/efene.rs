//! Hello-world snippet for `ProgrammingLanguageName::Efene`.
//!
//! Imported from leachim6/hello-world (`e/Efene.efene` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Efene.efene>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Efene => "Efene",
    versions: [Efene => "Efene"],
    extensions: [Efene => "efene"],
}

const EFENE: &str = concat!(
    r#"@public
run = fn () {
    io.format(""#,
    output_for_hello_world!(),
    r#"~n")
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Efene,
        extensions: &[FileExtension::Efene],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Efene,
            syntax: EFENE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EFENE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"@public
run = fn () {
    io.format(""#,
                            output_for_hello_world!(),
                            r#"~n")
}"#
                        ),
                    )],
                )
            },
        }],
    };
