//! Hello-world snippet for `ProgrammingLanguageName::AlgolW`.
//!
//! Imported from leachim6/hello-world (`a/ALGOL W.algol` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ALGOL%20W.algol>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AlgolW => "ALGOL W",
    versions: [AlgolW => "ALGOL W"],
    extensions: [Algol => "algol"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AlgolW,
        extensions: &[FileExtension::Algol],
        categories: &[Compiled, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AlgolW,
            syntax: concat!(
                r#"begin
    write( ""#,
                output_for_hello_world!(),
                r#"" )
end."#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"begin
    write( ""#,
                    output_for_hello_world!(),
                    r#"" )
end."#
                ),
            )],
        }],
    };
