//! Hello-world snippet for `ProgrammingLanguageName::Incal`.
//!
//! Imported from leachim6/hello-world (`i/INCAL.incal` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/INCAL.incal>
//! - <https://esolangs.org/wiki/INCAL> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Reflective;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Incal => "INCAL",
    versions: [Incal => "INCAL"],
    extensions: [Incal => "incal"],
}

const INCAL: &str = r"send,66#111#102#<101#42#93#101#120#102#110#43#
die";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Incal,
        extensions: &[FileExtension::Incal],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Reflective],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Incal,
            syntax: INCAL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    INCAL,
                    [(
                        TokenKind::Plain,
                        r"send,66#111#102#<101#42#93#101#120#102#110#43#
die",
                    )],
                )
            },
        }],
    };
