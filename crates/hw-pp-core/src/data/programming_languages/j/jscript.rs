//! Hello-world snippet for `ProgrammingLanguageName::Jscript`.
//!
//! Imported from leachim6/hello-world (`j/JScript.js` at `a152253`),
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
    language: Jscript => "JScript",
    versions: [Jscript => "JScript"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jscript,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jscript,
            syntax: concat!(r#"WScript.Echo(""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[
                (TokenKind::Plain, r"WScript"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"Echo"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r");"),
            ],
        }],
    };
