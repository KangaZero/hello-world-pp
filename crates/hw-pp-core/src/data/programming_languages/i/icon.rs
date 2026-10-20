//! Hello-world snippet for `ProgrammingLanguageName::Icon`.
//!
//! Imported from leachim6/hello-world (`i/Icon.icn` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/Icon.icn>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Historical, Scripting},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::{Icon, Snobol},
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Icon => "Icon",
    versions: [Icon => "Icon"],
    extensions: [Icn => "icn"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Icon,
        extensions: &[FileExtension::Icn],
        categories: &[Scripting, Historical],
        families: &[Icon, Snobol],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Icon,
            syntax: concat!(
                r#"procedure main()
    write(""#,
                output_for_hello_world!(),
                r#"");
end"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"procedure main()
    write(""#,
                    output_for_hello_world!(),
                    r#"");
end"#
                ),
            )],
        }],
    };
