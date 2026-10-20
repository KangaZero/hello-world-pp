//! Hello-world snippet for `ProgrammingLanguageName::Unicon`.
//!
//! Imported from leachim6/hello-world (`u/Unicon.icn` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/u/Unicon.icn>

use crate::data::programming_languages::{
    ProgrammingLanguage,
    ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::{Icon, Snobol},
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Unicon => "Unicon",
    versions: [Unicon => "Unicon"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Unicon,
        extensions: &[],
        categories: &[Scripting],
        families: &[Icon, Snobol],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Unicon,
            syntax: concat!(
                r#"procedure main()
    write(""#,
                output_for_hello_world!(),
                r#"")
end"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"procedure main()
    write(""#,
                    output_for_hello_world!(),
                    r#"")
end"#
                ),
            )],
        }],
    };
