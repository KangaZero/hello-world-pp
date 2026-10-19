//! Hello-world snippet for `ProgrammingLanguageName::Parasail`.
//!
//! Imported from leachim6/hello-world (`p/ParaSail.psi` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/ParaSail.psi>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Parasail => "ParaSail",
    versions: [Parasail => "ParaSail"],
    extensions: [Psi => "psi"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Parasail,
        extensions: &[FileExtension::Psi],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Parasail,
            syntax: concat!(
                r#"func Hello_World(var IO) is
    IO.Println(""#,
                output_for_hello_world!(),
                r#"");
end func Hello_World;"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"func Hello_World(var IO) is
    IO.Println(""#,
                    output_for_hello_world!(),
                    r#"");
end func Hello_World;"#
                ),
            )],
        }],
    };
