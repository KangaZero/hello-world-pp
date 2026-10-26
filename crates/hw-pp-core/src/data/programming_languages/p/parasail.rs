//! Hello-world snippet for `ProgrammingLanguageName::Parasail`.
//!
//! Imported from leachim6/hello-world (`p/ParaSail.psi` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/ParaSail.psi>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Pascal, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Parasail => "ParaSail",
    versions: [Parasail => "ParaSail"],
    extensions: [Psi => "psi"],
}

const PARASAIL: &str = concat!(
    r#"func Hello_World(var IO) is
    IO.Println(""#,
    output_for_hello_world!(),
    r#"");
end func Hello_World;"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Parasail,
        extensions: &[FileExtension::Psi],
        categories: &[Compiled],
        families: &[Pascal],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Parasail,
            syntax: PARASAIL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PARASAIL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"func Hello_World(var IO) is
    IO.Println(""#,
                            output_for_hello_world!(),
                            r#"");
end func Hello_World;"#
                        ),
                    )],
                )
            },
        }],
    };
