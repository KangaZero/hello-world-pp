//! Hello-world snippet for `ProgrammingLanguageName::Jscript`.
//!
//! Imported from leachim6/hello-world (`j/JScript.js` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/JScript.js>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::JavaScript, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Jscript => "JScript",
    versions: [Jscript => "JScript"],
    extensions: [],
}

const JSCRIPT: &str = concat!(r#"WScript.Echo(""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jscript,
        extensions: &[],
        categories: &[Scripting],
        families: &[JavaScript],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jscript,
            syntax: JSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    JSCRIPT,
                    [
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
                )
            },
        }],
    };
