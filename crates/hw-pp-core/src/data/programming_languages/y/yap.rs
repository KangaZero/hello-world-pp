//! Hello-world snippet for `ProgrammingLanguageName::Yap`.
//!
//! Imported from leachim6/hello-world (PR #1618 `y/YAP.yap`),
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
    language: Yap => "YAP",
    versions: [Yap => "YAP"],
    extensions: [Yap => "yap"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Yap,
        extensions: &[FileExtension::Yap],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Yap,
            syntax: concat!(
                r":- initialization(main).

main :-
    write('",
                output_for_hello_world!(),
                r"'), nl."
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r":- initialization(main).

main :-
    write('",
                    output_for_hello_world!(),
                    r"'), nl."
                ),
            )],
        }],
    };
