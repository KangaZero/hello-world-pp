//! Hello-world snippet for `ProgrammingLanguageName::Idl`.
//!
//! Imported from leachim6/hello-world (`i/IDL` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Idl => "IDL",
    versions: [Idl => "IDL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Idl,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Idl,
            syntax: concat!(
                r#"print, ""#,
                output_for_hello_world!(),
                r#""
end"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"print, ""#,
                    output_for_hello_world!(),
                    r#""
end"#
                ),
            )],
        }],
    };
