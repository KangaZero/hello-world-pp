//! Hello-world snippet for `ProgrammingLanguageName::Refal`.
//!
//! Imported from leachim6/hello-world (`r/Refal.ref` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Refal.ref>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Refal => "Refal",
    versions: [Refal => "Refal"],
    extensions: [Ref => "ref"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Refal,
        extensions: &[FileExtension::Ref],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Refal,
            syntax: concat!(
                r"$ENTRY Go { = <Hello>;}
Hello {
   = <Prout '",
                output_for_hello_world!(),
                r"'>;
}"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"$ENTRY Go { = <Hello>;}
Hello {
   = <Prout '",
                    output_for_hello_world!(),
                    r"'>;
}"
                ),
            )],
        }],
    };
