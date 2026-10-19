//! Hello-world snippet for `ProgrammingLanguageName::Cor`.
//!
//! Imported from leachim6/hello-world (`c/Cor.cor` at `a152253`),
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
    language: Cor => "Cor",
    versions: [Cor => "Cor"],
    extensions: [Cor => "cor"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cor,
        extensions: &[FileExtension::Cor],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cor,
            syntax: concat!(
                r#"func main() console.log(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"func main() console.log(""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
