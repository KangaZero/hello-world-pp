//! Hello-world snippet for `ProgrammingLanguageName::Franceprog`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#FrancePROG>
//! - <https://esolangs.org/wiki/FrancePROG>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Franceprog => "FrancePROG",
    versions: [Franceprog => "FrancePROG"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Franceprog,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Franceprog,
            syntax: concat!(
                r"Alors en gros, on va commencer un programme.
Ce serait bien si on écrivait «",
                output_for_hello_world!(),
                r"».
Après, on arrête le programme un instant! C'est bien ça."
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"Alors en gros, on va commencer un programme.
Ce serait bien si on écrivait «",
                    output_for_hello_world!(),
                    r"».
Après, on arrête le programme un instant! C'est bien ça."
                ),
            )],
        }],
    };
