//! Hello-world snippet for `ProgrammingLanguageName::Ufe65Ufe64Ufe65`.
//!
//! Imported from leachim6/hello-world (`#/﹥﹤﹥` at `a152253`),
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
    language: Ufe65Ufe64Ufe65 => "﹥﹤﹥",
    versions: [Ufe65Ufe64Ufe65 => "﹥﹤﹥"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ufe65Ufe64Ufe65,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ufe65Ufe64Ufe65,
            syntax: concat!(
                r#"""#,
                output_for_hello_world!(),
                r#""r\
          o;!?l<"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"""#,
                    output_for_hello_world!(),
                    r#""r\
          o;!?l<"#
                ),
            )],
        }],
    };
