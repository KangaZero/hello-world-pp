//! Hello-world snippet for `ProgrammingLanguageName::Bloop`.
//!
//! Imported from leachim6/hello-world (`b/Bloop.bloop` at `a152253`),
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
    language: Bloop => "Bloop",
    versions: [Bloop => "Bloop"],
    extensions: [Bloop => "bloop"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bloop,
        extensions: &[FileExtension::Bloop],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bloop,
            syntax: concat!(
                r"DEFINE PROCEDURE ''HELLO-WORLD'' [N]:
BLOCK 0: BEGIN
     PRINT['",
                output_for_hello_world!(),
                r"']
BLOCK 0: END.
HELLO-WORLD[1]; "
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"DEFINE PROCEDURE ''HELLO-WORLD'' [N]:
BLOCK 0: BEGIN
     PRINT['",
                    output_for_hello_world!(),
                    r"']
BLOCK 0: END.
HELLO-WORLD[1]; "
                ),
            )],
        }],
    };
