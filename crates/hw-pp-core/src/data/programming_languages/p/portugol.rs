//! Hello-world snippet for `ProgrammingLanguageName::Portugol`.
//!
//! Imported from leachim6/hello-world (`p/Portugol.por` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Portugol.por>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Portugol => "Portugol",
    versions: [Portugol => "Portugol"],
    extensions: [Por => "por"],
}

const PORTUGOL: &str = concat!(
    r#"algoritmo ""#,
    output_for_hello_world!(),
    r#""

var
 
inicio
escreva (""#,
    output_for_hello_world!(),
    r#"")
fimalgoritmo"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Portugol,
        extensions: &[FileExtension::Por],
        categories: &[Educational],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Portugol,
            syntax: PORTUGOL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PORTUGOL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"algoritmo ""#,
                            output_for_hello_world!(),
                            r#""

var
 
inicio
escreva (""#,
                            output_for_hello_world!(),
                            r#"")
fimalgoritmo"#
                        ),
                    )],
                )
            },
        }],
    };
