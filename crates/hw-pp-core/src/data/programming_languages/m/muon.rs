//! Hello-world snippet for `ProgrammingLanguageName::Muon`.
//!
//! Imported from leachim6/hello-world (`m/Muon.mu` at `a152253`),
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
    language: Muon => "Muon",
    versions: [Muon => "Muon"],
    extensions: [Mu => "mu"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Muon,
        extensions: &[FileExtension::Mu],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Muon,
            syntax: concat!(
                r#"printf(fmt cstring) int #Foreign("printf") #VarArgs

main() {
	printf(""#,
                output_for_hello_world!(),
                r#"")
}"#
            ),
            tokenized_syntax: &[(
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
        }],
    };
