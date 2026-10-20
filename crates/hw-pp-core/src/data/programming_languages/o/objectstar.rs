//! Hello-world snippet for `ProgrammingLanguageName::Objectstar`.
//!
//! Imported from leachim6/hello-world (`o/ObjectStar` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/ObjectStar>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Objectstar => "ObjectStar",
    versions: [Objectstar => "ObjectStar"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Objectstar,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Objectstar,
            syntax: concat!(r"CALL ENDMSG('", output_for_hello_world!(), r"');"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"CALL ENDMSG('", output_for_hello_world!(), r"');"),
            )],
        }],
    };
