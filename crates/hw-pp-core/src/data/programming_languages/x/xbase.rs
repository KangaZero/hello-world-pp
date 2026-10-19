//! Hello-world snippet for `ProgrammingLanguageName::Xbase`.
//!
//! Imported from leachim6/hello-world (`x/XBase.dbf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/XBase.dbf>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Historical, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Xbase => "XBase",
    versions: [Xbase => "XBase"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xbase,
        extensions: &[],
        categories: &[Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xbase,
            syntax: concat!(
                r"do while .t.
 ? '",
                output_for_hello_world!(),
                r"'
enddo"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"do while .t.
 ? '",
                    output_for_hello_world!(),
                    r"'
enddo"
                ),
            )],
        }],
    };
