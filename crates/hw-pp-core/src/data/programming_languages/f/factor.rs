//! Hello-world snippet for `ProgrammingLanguageName::Factor`.
//!
//! Imported from leachim6/hello-world (`f/Factor.factor` at `a152253`),
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
    language: Factor => "Factor",
    versions: [Factor => "Factor"],
    extensions: [Factor => "factor"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Factor,
        extensions: &[FileExtension::Factor],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Factor,
            syntax: concat!(
                r#"USING: io ;
""#,
                output_for_hello_world!(),
                r#"" print
"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"USING: io ;
""#,
                    output_for_hello_world!(),
                    r#"" print
"#
                ),
            )],
        }],
    };
