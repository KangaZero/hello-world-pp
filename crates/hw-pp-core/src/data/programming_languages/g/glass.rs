//! Hello-world snippet for `ProgrammingLanguageName::Glass`.
//!
//! Imported from leachim6/hello-world (`g/Glass.glass` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Glass.glass>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Glass => "Glass",
    versions: [Glass => "Glass"],
    extensions: [Glass => "glass"],
}

const GLASS: &str = concat!(
    r#"+{M[m(_o)O!""#,
    output_for_hello_world!(),
    r#""(_o)o.?]}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Glass,
        extensions: &[FileExtension::Glass],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Glass,
            syntax: GLASS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GLASS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"+{M[m(_o)O!""#,
                            output_for_hello_world!(),
                            r#""(_o)o.?]}"#
                        ),
                    )],
                )
            },
        }],
    };
