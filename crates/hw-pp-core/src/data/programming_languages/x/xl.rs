//! Hello-world snippet for `ProgrammingLanguageName::Xl`.
//!
//! Imported from leachim6/hello-world (`x/XL.xl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/XL.xl>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Xl => "XL",
    versions: [Xl => "XL"],
    extensions: [Xl => "xl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xl,
        extensions: &[FileExtension::Xl],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xl,
            syntax: concat!(
                r#"// XL programming language: http://xlr.sourceforge.net
writeln ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"// XL programming language: http://xlr.sourceforge.net
writeln ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
