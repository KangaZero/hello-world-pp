//! Hello-world snippet for `ProgrammingLanguageName::FimPlusPlus`.
//!
//! Imported from leachim6/hello-world (`f/Fim++.fimpp` at `a152253`),
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
    language: FimPlusPlus => "Fim++",
    versions: [FimPlusPlus => "Fim++"],
    extensions: [Fimpp => "fimpp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FimPlusPlus,
        extensions: &[FileExtension::Fimpp],
        categories: &[],
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
