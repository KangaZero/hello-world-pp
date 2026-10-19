//! Hello-world snippet for `ProgrammingLanguageName::Processing`.
//!
//! Imported from leachim6/hello-world (`p/Processing.pde` at `a152253`),
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
    language: Processing => "Processing",
    versions: [Processing => "Processing"],
    extensions: [Pde => "pde"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Processing,
        extensions: &[FileExtension::Pde],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Processing,
            syntax: concat!(
                r#"size(128, 128);
background(0);
textAlign(CENTER, CENTER);
fill(255);
text(""#,
                output_for_hello_world!(),
                r#"", width / 2, height / 2);"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"size(128, 128);
background(0);
textAlign(CENTER, CENTER);
fill(255);
text(""#,
                    output_for_hello_world!(),
                    r#"", width / 2, height / 2);"#
                ),
            )],
        }],
    };
