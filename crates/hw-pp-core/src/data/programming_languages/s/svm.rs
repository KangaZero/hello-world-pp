//! Hello-world snippet for `ProgrammingLanguageName::Svm`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#SVM>
//! - <https://esolangs.org/wiki/SVM>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Svm => "SVM",
    versions: [Svm => "SVM"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Svm,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Svm,
            syntax: concat!(
                r"## The ",
                output_for_hello_world!(),
                r" program in SVM code
AM =0000000C
CM $",
                output_for_hello_world!(),
                r",=0000000C,*00000000
DD *00000000,=0000000C
IC *00000000
EP"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"## The ",
                    output_for_hello_world!(),
                    r" program in SVM code
AM =0000000C
CM $",
                    output_for_hello_world!(),
                    r",=0000000C,*00000000
DD *00000000,=0000000C
IC *00000000
EP"
                ),
            )],
        }],
    };
