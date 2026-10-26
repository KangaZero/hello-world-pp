//! Hello-world snippet for `ProgrammingLanguageName::Ksp`.
//!
//! Imported from leachim6/hello-world (`k/KSP.ksp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/k/KSP.ksp>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ksp => "KSP",
    versions: [Ksp => "KSP"],
    extensions: [Ksp => "ksp"],
}

const KSP: &str = concat!(
    r#"on init
	message(""#,
    output_for_hello_world!(),
    r#"")
end on"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ksp,
        extensions: &[FileExtension::Ksp],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ksp,
            syntax: KSP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    KSP,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"on init
	message(""#,
                            output_for_hello_world!(),
                            r#"")
end on"#
                        ),
                    )],
                )
            },
        }],
    };
