//! Hello-world snippet for `ProgrammingLanguageName::Maxscript`.
//!
//! Imported from leachim6/hello-world (`m/MaxScript.ms` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/MaxScript.ms>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Maxscript => "MaxScript",
    versions: [Maxscript => "MaxScript"],
    extensions: [Ms => "ms"],
}

const MAXSCRIPT: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Maxscript,
        extensions: &[FileExtension::Ms],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Maxscript,
            syntax: MAXSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MAXSCRIPT,
                    [(
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
                )
            },
        }],
    };
