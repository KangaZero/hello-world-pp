//! Hello-world snippet for `ProgrammingLanguageName::KoberiC`.
//!
//! Imported from leachim6/hello-world (`k/Kobeři-C.koberice` at `a152253`),
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
    language: KoberiC => "Kobeři-C",
    versions: [KoberiC => "Kobeři-C"],
    extensions: [Koberice => "koberice"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::KoberiC,
        extensions: &[FileExtension::Koberice],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::KoberiC,
            syntax: concat!(
                r#"(int main()
    (print ""#,
                output_for_hello_world!(),
                r#"")
    (return 0))"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"(int main()
    (print ""#,
                    output_for_hello_world!(),
                    r#"")
    (return 0))"#
                ),
            )],
        }],
    };
