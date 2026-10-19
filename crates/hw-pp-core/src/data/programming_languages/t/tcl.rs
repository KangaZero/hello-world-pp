//! Hello-world snippet for `ProgrammingLanguageName::Tcl`.
//!
//! Imported from leachim6/hello-world (`t/Tcl.tcl` at `a152253`),
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
    language: Tcl => "Tcl",
    versions: [Tcl => "Tcl"],
    extensions: [Tcl => "tcl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tcl,
        extensions: &[FileExtension::Tcl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tcl,
            syntax: concat!(r#"puts ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"puts"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
            ],
        }],
    };
