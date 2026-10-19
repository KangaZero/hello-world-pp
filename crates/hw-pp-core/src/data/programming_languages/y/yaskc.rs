//! Hello-world snippet for `ProgrammingLanguageName::Yaskc`.
//!
//! Imported from leachim6/hello-world (`y/YASKC.yaskc` at `a152253`),
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
    language: Yaskc => "YASKC",
    versions: [Yaskc => "YASKC"],
    extensions: [Yaskc => "yaskc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Yaskc,
        extensions: &[FileExtension::Yaskc],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Yaskc,
            syntax: concat!(
                r"using universal delay 200
using loop False

typewrite ",
                output_for_hello_world!()
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"using universal delay 200
using loop False

typewrite ",
                    output_for_hello_world!()
                ),
            )],
        }],
    };
