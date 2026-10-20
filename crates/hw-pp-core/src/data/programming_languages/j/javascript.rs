//! Hello-world snippets for `ProgrammingLanguageName::JavaScript`.
//!
//! Sources:
//! - <https://developer.mozilla.org/en-US/docs/Web/API/console/log_static>
//! - <https://developer.mozilla.org/en-US/docs/Learn_web_development/Core/Scripting/What_is_JavaScript>
//! - <https://262.ecma-international.org/11.0/>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::JavaScript, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: JavaScript => "JavaScript",
    versions: [JavaScriptEs2020 => "ES2020"],
    extensions: [Js => "js", Mjs => "mjs", Cjs => "cjs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::JavaScript,
        extensions: &[FileExtension::Js, FileExtension::Mjs, FileExtension::Cjs],
        categories: &[Scripting],
        families: &[JavaScript],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::JavaScriptEs2020,
            syntax: concat!(r#"console.log(""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[
                (TokenKind::Type, "console"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Function, "log"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ");"),
            ],
        }],
    };
