//! Hello-world snippet for `ProgrammingLanguageName::Modiscript`.
//!
//! Imported from leachim6/hello-world (`m/ModiScript.chai` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/ModiScript.chai>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Modiscript => "ModiScript",
    versions: [Modiscript => "ModiScript"],
    extensions: [Chai => "chai"],
}

const MODISCRIPT: &str = concat!(
    r#"Mitrooon
    bhaiyo aur behno ""#,
    output_for_hello_world!(),
    r#""
Achhe din aa gaye"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Modiscript,
        extensions: &[FileExtension::Chai],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Modiscript,
            syntax: MODISCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MODISCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"Mitrooon
    bhaiyo aur behno ""#,
                            output_for_hello_world!(),
                            r#""
Achhe din aa gaye"#
                        ),
                    )],
                )
            },
        }],
    };
