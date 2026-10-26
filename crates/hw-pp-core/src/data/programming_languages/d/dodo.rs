//! Hello-world snippet for `ProgrammingLanguageName::Dodo`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#DODO>
//! - <https://esolangs.org/wiki/DODO>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Dodo => "DODO",
    versions: [Dodo => "DODO"],
    extensions: [],
}

const DODO: &str = concat!(r#"""#, output_for_hello_world!(), r#"" OUTS DO"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dodo,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dodo,
            syntax: DODO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DODO,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#"" OUTS DO"#),
                    )],
                )
            },
        }],
    };
