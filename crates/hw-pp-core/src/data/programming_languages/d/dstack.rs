//! Hello-world snippet for `ProgrammingLanguageName::Dstack`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#DStack>
//! - <https://esolangs.org/wiki/DStack>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dstack => "DStack",
    versions: [Dstack => "DStack"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dstack,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dstack,
            syntax: concat!(
                r"@0
",
                output_for_hello_world!(),
                r"
@
ad"
            ),
            tokenized_syntax: &[(
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
        }],
    };
