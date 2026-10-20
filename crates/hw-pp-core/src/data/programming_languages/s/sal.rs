//! Hello-world snippet for `ProgrammingLanguageName::Sal`.
//!
//! Imported from leachim6/hello-world (`s/Sal.s` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Sal.s>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Sal => "Sal",
    versions: [Sal => "Sal"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sal,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sal,
            syntax: concat!(
                r#"proc main()
    loop
        WriteLine(""#,
                output_for_hello_world!(),
                r#"")
    endloop
end"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"proc main()
    loop
        WriteLine(""#,
                    output_for_hello_world!(),
                    r#"")
    endloop
end"#
                ),
            )],
        }],
    };
