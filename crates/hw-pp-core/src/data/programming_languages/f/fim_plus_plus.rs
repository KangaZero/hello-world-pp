//! Hello-world snippet for `ProgrammingLanguageName::FimPlusPlus`.
//!
//! Imported from leachim6/hello-world (`f/Fim++.fimpp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Fim%2B%2B.fimpp>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: FimPlusPlus => "Fim++",
    versions: [FimPlusPlus => "Fim++"],
    extensions: [Fimpp => "fimpp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FimPlusPlus,
        extensions: &[FileExtension::Fimpp],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FimPlusPlus,
            syntax: concat!(
                r"Dear Princess Celestia: ",
                output_for_hello_world!(),
                r"

Today I learned how to say ",
                output_for_hello_world!(),
                r#"
I said ""#,
                output_for_hello_world!(),
                r#"".
That's all about how to say "#,
                output_for_hello_world!(),
                r".

Your faithful student, Twilight Sparkle. "
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"Dear Princess Celestia: ",
                    output_for_hello_world!(),
                    r"

Today I learned how to say ",
                    output_for_hello_world!(),
                    r#"
I said ""#,
                    output_for_hello_world!(),
                    r#"".
That's all about how to say "#,
                    output_for_hello_world!(),
                    r".

Your faithful student, Twilight Sparkle. "
                ),
            )],
        }],
    };
