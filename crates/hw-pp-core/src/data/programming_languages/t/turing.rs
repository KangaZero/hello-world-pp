//! Hello-world snippets for `ProgrammingLanguageName::Turing`.
//!
//! Imported from leachim6/hello-world (`t/Turing.oot`, `t/Turing.t` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Turing.oot>
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Turing.t>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Algol, ProgrammingLanguageName,
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
        categories: &[Educational],
        families: &[Algol],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::TuringOot,
                syntax: concat!(r#"put ""#, output_for_hello_world!(), r#"""#),
                tokenized_syntax: &[
                    (TokenKind::Keyword, r"put"),
                    (TokenKind::Plain, r" "),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::TuringT,
                syntax: concat!(r#"put ""#, output_for_hello_world!(), r#"""#),
                tokenized_syntax: &[
                    (TokenKind::Keyword, r"put"),
                    (TokenKind::Plain, r" "),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                ],
            },
        ],
    };
