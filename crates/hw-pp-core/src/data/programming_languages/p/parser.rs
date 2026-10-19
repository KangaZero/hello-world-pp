//! Hello-world snippet for `ProgrammingLanguageName::Parser`.
//!
//! Imported from leachim6/hello-world (`p/Parser.p` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Parser => "Parser",
    versions: [Parser => "Parser"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Parser,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Parser,
            syntax: concat!(
                r#"@main[]
  ^rem{Will print ""#,
                output_for_hello_world!(),
                r#"" when run as CGI script}
  $hello["#,
                output_for_hello_world!(),
                r"]
  $result[$hello]"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"@main[]
  ^rem{Will print ""#,
                    output_for_hello_world!(),
                    r#"" when run as CGI script}
  $hello["#,
                    output_for_hello_world!(),
                    r"]
  $result[$hello]"
                ),
            )],
        }],
    };
