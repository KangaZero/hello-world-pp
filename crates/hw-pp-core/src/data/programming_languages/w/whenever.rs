//! Hello-world snippet for `ProgrammingLanguageName::Whenever`.
//!
//! Imported from leachim6/hello-world (`w/Whenever.whenever` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/w/Whenever.whenever>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Whenever => "Whenever",
    versions: [Whenever => "Whenever"],
    extensions: [Whenever => "whenever"],
}

const WHENEVER: &str = concat!(r#"1 print(""#, output_for_hello_world!(), r#""); "#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Whenever,
        extensions: &[FileExtension::Whenever],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Whenever,
            syntax: WHENEVER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    WHENEVER,
                    [(
                        TokenKind::Plain,
                        concat!(r#"1 print(""#, output_for_hello_world!(), r#""); "#),
                    )],
                )
            },
        }],
    };
