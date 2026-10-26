//! Hello-world snippet for `ProgrammingLanguageName::Xbase`.
//!
//! Imported from leachim6/hello-world (`x/XBase.dbf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/XBase.dbf>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Historical, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::XBase, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Xbase => "XBase",
    versions: [Xbase => "XBase"],
    extensions: [],
}

const XBASE: &str = concat!(
    r"do while .t.
 ? '",
    output_for_hello_world!(),
    r"'
enddo"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xbase,
        extensions: &[],
        categories: &[Historical],
        families: &[XBase],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xbase,
            syntax: XBASE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    XBASE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"do while .t.
 ? '",
                            output_for_hello_world!(),
                            r"'
enddo"
                        ),
                    )],
                )
            },
        }],
    };
