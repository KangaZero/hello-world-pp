//! Hello-world snippet for `ProgrammingLanguageName::Objectstar`.
//!
//! Imported from leachim6/hello-world (`o/ObjectStar` at `a152253`),
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
    language: Objectstar => "ObjectStar",
    versions: [Objectstar => "ObjectStar"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Objectstar,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Objectstar,
            syntax: concat!(r"CALL ENDMSG('", output_for_hello_world!(), r"');"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"CALL ENDMSG('", output_for_hello_world!(), r"');"),
            )],
        }],
    };
