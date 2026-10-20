//! Hello-world snippet for `ProgrammingLanguageName::Trumpscript`.
//!
//! Imported from leachim6/hello-world (`t/TrumpScript.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/TrumpScript.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Esolang],
        families: &[],
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
