//! Hello-world snippet for `ProgrammingLanguageName::Fstar`.
//!
//! Imported from leachim6/hello-world (`f/FStar.fst` at `a152253`),
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
    language: Fstar => "FStar",
    versions: [Fstar => "FStar"],
    extensions: [Fst => "fst"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fstar,
        extensions: &[FileExtension::Fst],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fstar,
            syntax: concat!(
                r#"module Hello

let main = FStar.IO.print_string ""#,
                output_for_hello_world!(),
                r#"\n""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"module Hello

let main = FStar.IO.print_string ""#,
                    output_for_hello_world!(),
                    r#"\n""#
                ),
            )],
        }],
    };
