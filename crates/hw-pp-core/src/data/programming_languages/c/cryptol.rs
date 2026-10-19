//! Hello-world snippet for `ProgrammingLanguageName::Cryptol`.
//!
//! Imported from leachim6/hello-world (`c/Cryptol` at `a152253`),
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
    language: Cryptol => "Cryptol",
    versions: [Cryptol => "Cryptol"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cryptol,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cryptol,
            syntax: concat!(
                r#":set ascii=on
""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#":set ascii=on
""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
