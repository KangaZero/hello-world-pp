//! Hello-world snippet for `ProgrammingLanguageName::Muf`.
//!
//! Imported from leachim6/hello-world (`m/Muf.muf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Muf.muf>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Muf => "Muf",
    versions: [Muf => "Muf"],
    extensions: [Muf => "muf"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Muf,
        extensions: &[FileExtension::Muf],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Muf,
            syntax: concat!(
                r#": main
    me @ ""#,
                output_for_hello_world!(),
                r#"" notify
;"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#": main
    me @ ""#,
                    output_for_hello_world!(),
                    r#"" notify
;"#
                ),
            )],
        }],
    };
