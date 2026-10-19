//! Hello-world snippet for `ProgrammingLanguageName::Spss`.
//!
//! Imported from leachim6/hello-world (`s/SPSS.spss` at `a152253`),
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
    language: Spss => "SPSS",
    versions: [Spss => "SPSS"],
    extensions: [Spss => "spss"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Spss,
        extensions: &[FileExtension::Spss],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Spss,
            syntax: concat!(
                r#"BEGIN PROGRAM.
print ""#,
                output_for_hello_world!(),
                r#""
END PROGRAM."#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"BEGIN PROGRAM.
print ""#,
                    output_for_hello_world!(),
                    r#""
END PROGRAM."#
                ),
            )],
        }],
    };
