//! Hello-world snippet for `ProgrammingLanguageName::CPlusPlusPlus`.
//!
//! Imported from leachim6/hello-world (`c/C+++` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/C%2B%2B%2B>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: CPlusPlusPlus => "C+++",
    versions: [CPlusPlusPlus => "C+++"],
    extensions: [],
}

const C_PLUS_PLUS_PLUS: &str = concat!(
    r#"integer new main()
{
   println.format(""#,
    output_for_hello_world!(),
    r#"");
   return 0;
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CPlusPlusPlus,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CPlusPlusPlus,
            syntax: C_PLUS_PLUS_PLUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    C_PLUS_PLUS_PLUS,
                    [(
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
                )
            },
        }],
    };
