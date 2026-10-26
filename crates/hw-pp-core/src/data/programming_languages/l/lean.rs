//! Hello-world snippet for `ProgrammingLanguageName::Lean`.
//!
//! Imported from leachim6/hello-world (`l/Lean.lean` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Lean.lean>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::ProofAssistant,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Lean => "Lean",
    versions: [Lean => "Lean"],
    extensions: [Lean => "lean"],
}

const LEAN: &str = concat!(r#"#print ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lean,
        extensions: &[FileExtension::Lean],
        categories: &[ProofAssistant],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lean,
            syntax: LEAN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LEAN,
                    [
                        (TokenKind::Plain, r"#"),
                        (TokenKind::Function, r"print"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };
