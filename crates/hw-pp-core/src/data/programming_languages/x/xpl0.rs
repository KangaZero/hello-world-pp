//! Hello-world snippet for `ProgrammingLanguageName::Xpl0`.
//!
//! Imported from leachim6/hello-world (`x/XPL0.xpl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/XPL0.xpl>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::PlI, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Xpl0 => "XPL0",
    versions: [Xpl0 => "XPL0"],
    extensions: [Xpl => "xpl"],
}

const XPL0: &str = concat!(
    r#"code Text=12;
Text(0, ""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xpl0,
        extensions: &[FileExtension::Xpl],
        categories: &[Compiled],
        families: &[PlI],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xpl0,
            syntax: XPL0,
            tokenized_syntax: &const {
                checked_token_lengths(
                    XPL0,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"code Text=12;
Text(0, ""#,
                            output_for_hello_world!(),
                            r#"")"#
                        ),
                    )],
                )
            },
        }],
    };
