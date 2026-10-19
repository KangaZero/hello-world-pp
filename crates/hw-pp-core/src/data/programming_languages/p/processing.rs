//! Hello-world snippet for `ProgrammingLanguageName::Processing`.
//!
//! Imported from leachim6/hello-world (`p/Processing.pde` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Processing.pde>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Educational],
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
