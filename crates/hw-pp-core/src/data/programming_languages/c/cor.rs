//! Hello-world snippet for `ProgrammingLanguageName::Cor`.
//!
//! Imported from leachim6/hello-world (`c/Cor.cor` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cor.cor>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Cor => "Cor",
    versions: [Cor => "Cor"],
    extensions: [Cor => "cor"],
}

const COR: &str = concat!(
    r#"func main() console.log(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cor,
        extensions: &[FileExtension::Cor],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cor,
            syntax: COR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    COR,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"func main() console.log(""#,
                            output_for_hello_world!(),
                            r#"")"#
                        ),
                    )],
                )
            },
        }],
    };
