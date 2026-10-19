//! Hello-world snippets for `ProgrammingLanguageName::Prolog`.
//!
//! Sources:
//! - <https://www.swi-prolog.org/pldoc/man?section=consulting>
//! - <https://www.swi-prolog.org/pldoc/man?section=initgoal>
//! - <https://en.wikipedia.org/wiki/Prolog>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Prolog => "Prolog",
    versions: [IsoProlog => "ISO Prolog"],
    extensions: [Pl => "pl", Pro => "pro"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Prolog,
        extensions: &[FileExtension::Pl, FileExtension::Pro],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::IsoProlog,
            syntax: concat!(
                r":- initialization(main).

main :- write('",
                output_for_hello_world!(),
                r"'), nl, halt."
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, ":- initialization(main).\n\nmain :- "),
                (TokenKind::Function, "write"),
                (TokenKind::Plain, "("),
                (
                    TokenKind::String,
                    concat!("'", output_for_hello_world!(), "'"),
                ),
                (TokenKind::Plain, "), nl, halt."),
            ],
        }],
    };
