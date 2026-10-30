//! Hello-world snippet for `ProgrammingLanguageName::Purestack`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#PureStack>
//! - <https://esolangs.org/wiki/PureStack>

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Purestack => "PureStack",
    versions: [Purestack => "PureStack"],
    extensions: [],
}

const PURESTACK: &str = concat!(
    r#"""#,
    output_for_hello_world!(),
    r#""
!~"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Purestack,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Purestack,
            syntax: PURESTACK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PURESTACK,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"""#,
                            output_for_hello_world!(),
                            r#""
!~"#
                        ),
                    )],
                )
            },
        }],
    };
