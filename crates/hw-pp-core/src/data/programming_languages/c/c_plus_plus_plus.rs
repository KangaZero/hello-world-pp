//! Hello-world snippet for `ProgrammingLanguageName::CPlusPlusPlus`.
//!
//! Imported from leachim6/hello-world (`c/C+++` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: CPlusPlusPlus => "C+++",
    versions: [CPlusPlusPlus => "C+++"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CPlusPlusPlus,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CPlusPlusPlus,
            syntax: concat!(
                r#"integer new main()
{
   println.format(""#,
                output_for_hello_world!(),
                r#"");
   return 0;
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"integer new main()
{
   println.format(""#,
                    output_for_hello_world!(),
                    r#"");
   return 0;
}"#
                ),
            )],
        }],
    };
