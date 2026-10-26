//! Hello-world snippet for `ProgrammingLanguageName::JavaScript`.
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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: JavaScript => "JavaScript",
    versions: [JavaScriptEs2020 => "ES2020"],
    extensions: [Js => "js", Mjs => "mjs", Cjs => "cjs"],
}

const JAVA_SCRIPT_ES2020: &str = concat!(r#"console.log(""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::JavaScript,
        extensions: &[FileExtension::Js, FileExtension::Mjs, FileExtension::Cjs],
        categories: &[Scripting],
        families: &[JavaScript],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::JavaScriptEs2020,
            syntax: JAVA_SCRIPT_ES2020,
            tokenized_syntax: &const {
                checked_token_lengths(
                    JAVA_SCRIPT_ES2020,
                    [
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
                )
            },
        }],
    };
