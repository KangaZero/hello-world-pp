//! Hello-world snippet for `ProgrammingLanguageName::Tk`.
//!
//! Imported from leachim6/hello-world (`t/Tk.tk` at `a152253`),
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
    language: Tk => "Tk",
    versions: [Tk => "Tk"],
    extensions: [Tk => "tk"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tk,
        extensions: &[FileExtension::Tk],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tk,
            syntax: concat!(
                r#"label .l -text ""#,
                output_for_hello_world!(),
                r#""
pack .l"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"label .l -text ""#,
                    output_for_hello_world!(),
                    r#""
pack .l"#
                ),
            )],
        }],
    };
