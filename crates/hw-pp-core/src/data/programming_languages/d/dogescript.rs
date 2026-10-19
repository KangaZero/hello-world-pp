//! Hello-world snippet for `ProgrammingLanguageName::Dogescript`.
//!
//! Imported from leachim6/hello-world (`d/DogeScript.djs` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/DogeScript.djs>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dogescript => "DogeScript",
    versions: [Dogescript => "DogeScript"],
    extensions: [Djs => "djs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dogescript,
        extensions: &[FileExtension::Djs],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dogescript,
            syntax: concat!(
                r#"shh such hello dogescript very next-gen wow difficulty

plz console.loge with ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"shh such hello dogescript very next-gen wow difficulty

plz console.loge with ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
