//! Hello-world snippet for `ProgrammingLanguageName::Muf`.
//!
//! Imported from leachim6/hello-world (`m/Muf.muf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
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
