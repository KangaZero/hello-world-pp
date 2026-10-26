//! Hello-world snippet for `ProgrammingLanguageName::Prolog`.
//!
//! Sources:
//! - <https://www.swi-prolog.org/pldoc/man?section=consulting>
//! - <https://www.swi-prolog.org/pldoc/man?section=initgoal>
//! - <https://en.wikipedia.org/wiki/Prolog>
//! - <https://www.wikidata.org/wiki/Q163468> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Declarative, Logic};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Prolog, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Prolog => "Prolog",
    versions: [IsoProlog => "ISO Prolog"],
    extensions: [Pl => "pl", Pro => "pro"],
}

const ISO_PROLOG: &str = concat!(
    r":- initialization(main).

main :- write('",
    output_for_hello_world!(),
    r"'), nl, halt."
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Prolog,
        extensions: &[FileExtension::Pl, FileExtension::Pro],
        categories: &[Compiled],
        families: &[Prolog],
        paradigms: &[Declarative, Logic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::IsoProlog,
            syntax: ISO_PROLOG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ISO_PROLOG,
                    [
                        (TokenKind::Plain, ":- initialization(main).\n\nmain :- "),
                        (TokenKind::Function, "write"),
                        (TokenKind::Plain, "("),
                        (
                            TokenKind::String,
                            concat!("'", output_for_hello_world!(), "'"),
                        ),
                        (TokenKind::Plain, "), nl, halt."),
                    ],
                )
            },
        }],
    };
