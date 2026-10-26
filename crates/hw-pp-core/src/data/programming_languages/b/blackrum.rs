//! Hello-world snippet for `ProgrammingLanguageName::Blackrum`.
//!
//! Imported from leachim6/hello-world (`b/BlackRum.br` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/BlackRum.br>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Blackrum => "BlackRum",
    versions: [Blackrum => "BlackRum"],
    extensions: [Br => "br"],
}

const BLACKRUM: &str = concat!(r#"slur ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Blackrum,
        extensions: &[FileExtension::Br],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Blackrum,
            syntax: BLACKRUM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BLACKRUM,
                    [(
                        TokenKind::Plain,
                        concat!(r#"slur ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
