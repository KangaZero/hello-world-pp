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
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Dogescript => "DogeScript",
    versions: [Dogescript => "DogeScript"],
    extensions: [Djs => "djs"],
}

const DOGESCRIPT: &str = concat!(
    r#"shh such hello dogescript very next-gen wow difficulty

plz console.loge with ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dogescript,
        extensions: &[FileExtension::Djs],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dogescript,
            syntax: DOGESCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DOGESCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"shh such hello dogescript very next-gen wow difficulty

plz console.loge with ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };
