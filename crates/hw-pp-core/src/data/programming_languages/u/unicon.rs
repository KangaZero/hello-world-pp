//! Hello-world snippet for `ProgrammingLanguageName::Unicon`.
//!
//! Imported from leachim6/hello-world (`u/Unicon.icn` at `a152253`),
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
    language: Unicon => "Unicon",
    versions: [Unicon => "Unicon"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Unicon,
        extensions: &[],
        categories: &[],
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
