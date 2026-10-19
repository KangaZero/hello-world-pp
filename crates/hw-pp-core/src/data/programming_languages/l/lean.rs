//! Hello-world snippet for `ProgrammingLanguageName::Lean`.
//!
//! Imported from leachim6/hello-world (`l/Lean.lean` at `a152253`),
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
    language: Lean => "Lean",
    versions: [Lean => "Lean"],
    extensions: [Lean => "lean"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lean,
        extensions: &[FileExtension::Lean],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lean,
            syntax: concat!(r#"#print ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"#print ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
