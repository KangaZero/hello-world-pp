//! Hello-world snippet for `ProgrammingLanguageName::Betterave`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Betterave>
//! - <https://esolangs.org/wiki/Betterave>

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Betterave => "Betterave",
    versions: [Betterave => "Betterave"],
    extensions: [],
}

const BETTERAVE: &str = concat!(r#"$""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Betterave,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Betterave,
            syntax: BETTERAVE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BETTERAVE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"$""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
