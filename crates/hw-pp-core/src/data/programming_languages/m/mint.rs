//! Hello-world snippet for `ProgrammingLanguageName::Mint`.
//!
//! Imported from leachim6/hello-world (`m/Mint.mint` at `a152253`),
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
    language: Mint => "Mint",
    versions: [Mint => "Mint"],
    extensions: [Mint => "mint"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mint,
        extensions: &[FileExtension::Mint],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mint,
            syntax: concat!(
                r#"component Main {
  fun render : Html {
    <div>""#,
                output_for_hello_world!(),
                r#""</div>
  }
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"component Main {
  fun render : Html {
    <div>""#,
                    output_for_hello_world!(),
                    r#""</div>
  }
}"#
                ),
            )],
        }],
    };
