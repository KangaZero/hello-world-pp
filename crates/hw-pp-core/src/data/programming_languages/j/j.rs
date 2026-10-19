//! Hello-world snippet for `ProgrammingLanguageName::J`.
//!
//! Imported from leachim6/hello-world (`j/J.ijs` at `a152253`),
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
    language: J => "J",
    versions: [J => "J"],
    extensions: [Ijs => "ijs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::J,
        extensions: &[FileExtension::Ijs],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::J,
            syntax: concat!(
                r"#!/opt/local/bin/jc
echo '",
                output_for_hello_world!(),
                r"'
exit ''"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"#!/opt/local/bin/jc
echo '",
                    output_for_hello_world!(),
                    r"'
exit ''"
                ),
            )],
        }],
    };
