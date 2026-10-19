//! Hello-world snippet for `ProgrammingLanguageName::Alice`.
//!
//! Imported from leachim6/hello-world (`a/Alice.alice` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Alice => "Alice",
    versions: [Alice => "Alice"],
    extensions: [Alice => "alice"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Alice,
        extensions: &[FileExtension::Alice],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Alice,
            syntax: r#""dlroW olleH"d&O`@"#,
            tokenized_syntax: &[(TokenKind::Plain, r#""dlroW olleH"d&O`@"#)],
        }],
    };
