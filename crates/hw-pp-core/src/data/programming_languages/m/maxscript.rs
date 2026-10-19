//! Hello-world snippet for `ProgrammingLanguageName::Maxscript`.
//!
//! Imported from leachim6/hello-world (`m/MaxScript.ms` at `a152253`),
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
    language: Maxscript => "MaxScript",
    versions: [Maxscript => "MaxScript"],
    extensions: [Ms => "ms"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Maxscript,
        extensions: &[FileExtension::Ms],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Maxscript,
            syntax: concat!(
                r#"print ""#,
                output_for_hello_world!(),
                r#""
-- ""#,
                output_for_hello_world!(),
                r#""


-- Note that MAXScript is expression-based, so simply writing ""#,
                output_for_hello_world!(),
                r#"" is
-- sufficient to echo it for the reader. Like Haskell, all MAXScript expressions
-- *must* return values, even if they're unused."#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"print ""#,
                    output_for_hello_world!(),
                    r#""
-- ""#,
                    output_for_hello_world!(),
                    r#""


-- Note that MAXScript is expression-based, so simply writing ""#,
                    output_for_hello_world!(),
                    r#"" is
-- sufficient to echo it for the reader. Like Haskell, all MAXScript expressions
-- *must* return values, even if they're unused."#
                ),
            )],
        }],
    };
