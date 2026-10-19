//! Hello-world snippet for `ProgrammingLanguageName::Rpg`.
//!
//! Imported from leachim6/hello-world (`r/RPG.rpgle` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Rpg => "RPG",
    versions: [Rpg => "RPG"],
    extensions: [Rpgle => "rpgle"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rpg,
        extensions: &[FileExtension::Rpgle],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rpg,
            syntax: concat!(
                r"**free
dsply '",
                output_for_hello_world!(),
                r"';
return;"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"**free
dsply '",
                    output_for_hello_world!(),
                    r"';
return;"
                ),
            )],
        }],
    };
