//! Hello-world snippet for `ProgrammingLanguageName::Bloop`.
//!
//! Imported from leachim6/hello-world (`b/Bloop.bloop` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Bloop.bloop>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Bloop => "Bloop",
    versions: [Bloop => "Bloop"],
    extensions: [Bloop => "bloop"],
}

const BLOOP: &str = concat!(
    r"DEFINE PROCEDURE ''HELLO-WORLD'' [N]:
BLOCK 0: BEGIN
     PRINT['",
    output_for_hello_world!(),
    r"']
BLOCK 0: END.
HELLO-WORLD[1]; "
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bloop,
        extensions: &[FileExtension::Bloop],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bloop,
            syntax: BLOOP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BLOOP,
                    [(
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
                )
            },
        }],
    };
