//! Hello-world snippet for `ProgrammingLanguageName::Dup`.
//!
//! Imported from leachim6/hello-world (`d/DUP.dup` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/DUP.dup>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dup => "DUP",
    versions: [Dup => "DUP"],
    extensions: [Dup => "dup"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dup,
        extensions: &[FileExtension::Dup],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dup,
            syntax: concat!(
                r#"[\[^^>][$;,1+]#%%]p:
0$""#,
                output_for_hello_world!(),
                r#""p;!10,"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"[\[^^>][$;,1+]#%%]p:
0$""#,
                    output_for_hello_world!(),
                    r#""p;!10,"#
                ),
            )],
        }],
    };
