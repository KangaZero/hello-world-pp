//! Hello-world snippet for `ProgrammingLanguageName::Dstack`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#DStack>
//! - <https://esolangs.org/wiki/DStack>

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Dstack => "DStack",
    versions: [Dstack => "DStack"],
    extensions: [],
}

const DSTACK: &str = concat!(
    r"@0
",
    output_for_hello_world!(),
    r"
@
ad"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dstack,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dstack,
            syntax: DSTACK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DSTACK,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"@0
",
                            output_for_hello_world!(),
                            r"
@
ad"
                        ),
                    )],
                )
            },
        }],
    };
