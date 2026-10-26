//! Hello-world snippet for `ProgrammingLanguageName::Muon`.
//!
//! Imported from leachim6/hello-world (`m/Muon.mu` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Muon.mu>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Muon => "Muon",
    versions: [Muon => "Muon"],
    extensions: [Mu => "mu"],
}

const MUON: &str = concat!(
    r#"printf(fmt cstring) int #Foreign("printf") #VarArgs

main() {
	printf(""#,
    output_for_hello_world!(),
    r#"")
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Muon,
        extensions: &[FileExtension::Mu],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Muon,
            syntax: MUON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MUON,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"printf(fmt cstring) int #Foreign("printf") #VarArgs

main() {
	printf(""#,
                            output_for_hello_world!(),
                            r#"")
}"#
                        ),
                    )],
                )
            },
        }],
    };
