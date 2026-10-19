//! Hello-world snippet for `ProgrammingLanguageName::Novice`.
//!
//! Imported from leachim6/hello-world (`n/Novice.novice` at `a152253`),
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
    language: Novice => "Novice",
    versions: [Novice => "Novice"],
    extensions: [Novice => "novice"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Novice,
        extensions: &[FileExtension::Novice],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Novice,
            syntax: concat!(
                r"a
a-",
                output_for_hello_world!(),
                r"_*
",
                output_for_hello_world!(),
                r"_*"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"a
a-",
                    output_for_hello_world!(),
                    r"_*
",
                    output_for_hello_world!(),
                    r"_*"
                ),
            )],
        }],
    };
