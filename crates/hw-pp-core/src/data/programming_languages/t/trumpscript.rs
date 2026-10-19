//! Hello-world snippet for `ProgrammingLanguageName::Trumpscript`.
//!
//! Imported from leachim6/hello-world (`t/TrumpScript.py` at `a152253`),
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
    language: Trumpscript => "TrumpScript",
    versions: [Trumpscript => "TrumpScript"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Trumpscript,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Trumpscript,
            syntax: concat!(
                r#"say ""#,
                output_for_hello_world!(),
                r#""!
America is great."#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"say ""#,
                    output_for_hello_world!(),
                    r#""!
America is great."#
                ),
            )],
        }],
    };
