//! Hello-world snippet for `ProgrammingLanguageName::Icon`.
//!
//! Imported from leachim6/hello-world (`i/Icon.icn` at `a152253`),
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
    language: Icon => "Icon",
    versions: [Icon => "Icon"],
    extensions: [Icn => "icn"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Icon,
        extensions: &[FileExtension::Icn],
        categories: &[],
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
