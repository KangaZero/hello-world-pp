//! Hello-world snippet for `ProgrammingLanguageName::Obj`.
//!
//! Imported from leachim6/hello-world (`o/Obj.st` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Obj.st>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Obj => "Obj",
    versions: [Obj => "Obj"],
    extensions: [St => "st"],
}

const OBJ: &str = concat!(r"stdout println:'", output_for_hello_world!(), r"'.");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Obj,
        extensions: &[FileExtension::St],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Obj,
            syntax: OBJ,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OBJ,
                    [(
                        TokenKind::Plain,
                        concat!(r"stdout println:'", output_for_hello_world!(), r"'."),
                    )],
                )
            },
        }],
    };
