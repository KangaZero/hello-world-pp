//! Hello-world snippet for `ProgrammingLanguageName::Rivescript`.
//!
//! Imported from leachim6/hello-world (`r/RiveScript.rive` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/RiveScript.rive>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Rivescript => "RiveScript",
    versions: [Rivescript => "RiveScript"],
    extensions: [Rive => "rive"],
}

const RIVESCRIPT: &str = concat!(
    r"+ *
- ",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rivescript,
        extensions: &[FileExtension::Rive],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rivescript,
            syntax: RIVESCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RIVESCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"+ *
- ",
                            output_for_hello_world!()
                        ),
                    )],
                )
            },
        }],
    };
