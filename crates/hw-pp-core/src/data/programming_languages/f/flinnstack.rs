//! Hello-world snippet for `ProgrammingLanguageName::Flinnstack`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#FlinnStack>
//! - <https://esolangs.org/wiki/FlinnStack>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Flinnstack => "FlinnStack",
    versions: [Flinnstack => "FlinnStack"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Flinnstack,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Flinnstack,
            syntax: concat!(
                r#"push(""#,
                output_for_hello_world!(),
                r#"");
print;"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"push(""#,
                    output_for_hello_world!(),
                    r#"");
print;"#
                ),
            )],
        }],
    };
