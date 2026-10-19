//! Hello-world snippet for `ProgrammingLanguageName::Genie`.
//!
//! Imported from leachim6/hello-world (`g/Genie.gs` at `a152253`),
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
    language: Genie => "Genie",
    versions: [Genie => "Genie"],
    extensions: [Gs => "gs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Genie,
        extensions: &[FileExtension::Gs],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Genie,
            syntax: concat!(
                r#"[indent=2]
init
  print ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"[indent=2]
init
  print ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
