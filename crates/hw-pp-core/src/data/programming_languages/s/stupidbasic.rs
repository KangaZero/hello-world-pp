//! Hello-world snippet for `ProgrammingLanguageName::Stupidbasic`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#stupidBASIC>
//! - <https://esolangs.org/wiki/StupidBASIC>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Stupidbasic => "stupidBASIC",
    versions: [Stupidbasic => "stupidBASIC"],
    extensions: [],
}

const STUPIDBASIC: &str = concat!(
    r"program
 say ",
    output_for_hello_world!(),
    r"
end program"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Stupidbasic,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Stupidbasic,
            syntax: STUPIDBASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    STUPIDBASIC,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"program
 say ",
                            output_for_hello_world!(),
                            r"
end program"
                        ),
                    )],
                )
            },
        }],
    };
