//! Hello-world snippet for `ProgrammingLanguageName::Turing`.
//!
//! Imported from leachim6/hello-world (`t/Turing.oot` at `a152253`, `t/Turing.t` at `a152253`),
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
    language: Turing => "Turing",
    versions: [TuringOot => "Turing (.oot)", TuringT => "Turing (.t)"],
    extensions: [Oot => "oot"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Turing,
        extensions: &[FileExtension::Oot],
        categories: &[],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::TuringOot,
                syntax: concat!(r#"put ""#, output_for_hello_world!(), r#"""#),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(r#"put ""#, output_for_hello_world!(), r#"""#),
                )],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::TuringT,
                syntax: concat!(r#"put ""#, output_for_hello_world!(), r#"""#),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(r#"put ""#, output_for_hello_world!(), r#"""#),
                )],
            },
        ],
    };
