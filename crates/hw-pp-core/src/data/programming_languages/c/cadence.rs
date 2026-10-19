//! Hello-world snippet for `ProgrammingLanguageName::Cadence`.
//!
//! Imported from leachim6/hello-world (`c/Cadence.cdc` at `a152253`),
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
    language: Cadence => "Cadence",
    versions: [Cadence => "Cadence"],
    extensions: [Cdc => "cdc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cadence,
        extensions: &[FileExtension::Cdc],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cadence,
            syntax: concat!(
                r#"access(all) contract HelloWorld {
    access(all) let greeting: String
    init() {
        self.greeting = ""#,
                output_for_hello_world!(),
                r#""
    }

    access(all) fun hello(): String {
        return self.greeting
    }
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"access(all) contract HelloWorld {
    access(all) let greeting: String
    init() {
        self.greeting = ""#,
                    output_for_hello_world!(),
                    r#""
    }

    access(all) fun hello(): String {
        return self.greeting
    }
}"#
                ),
            )],
        }],
    };
