//! Hello-world snippet for `ProgrammingLanguageName::Chinesepython`.
//!
//! Imported from leachim6/hello-world (`c/ChinesePython.py` at `a152253`),
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
    language: Chinesepython => "ChinesePython",
    versions: [Chinesepython => "ChinesePython"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Chinesepython,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Chinesepython,
            syntax: concat!(r#"寫 ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"寫 ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
