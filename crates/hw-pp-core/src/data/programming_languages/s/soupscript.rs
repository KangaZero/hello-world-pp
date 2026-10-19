//! Hello-world snippet for `ProgrammingLanguageName::Soupscript`.
//!
//! Imported from leachim6/hello-world (`s/SoupScript.script` at `a152253`),
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
    language: Soupscript => "SoupScript",
    versions: [Soupscript => "SoupScript"],
    extensions: [Script => "script"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Soupscript,
        extensions: &[FileExtension::Script],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Soupscript,
            syntax: concat!(
                r"PrintLn ",
                output_for_hello_world!(),
                r"

Break"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"PrintLn ",
                    output_for_hello_world!(),
                    r"

Break"
                ),
            )],
        }],
    };
