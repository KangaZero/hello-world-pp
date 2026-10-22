//! Hello-world snippet for `ProgrammingLanguageName::Php`.
//!
//! Sources:
//! - <https://www.php.net/manual/en/tutorial.firstpage.php>
//! - <https://www.php.net/manual/en/function.echo.php>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::C, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Php => "PHP",
    versions: [Php8 => "PHP 8"],
    extensions: [Php => "php"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Php,
        extensions: &[FileExtension::Php],
        categories: &[Scripting],
        families: &[C],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Php8,
            syntax: concat!(
                r#"<?php
echo ""#,
                output_for_hello_world!(),
                r#"\n";"#
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, "<?php"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Function, "echo"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), r#"\n""#),
                ),
                (TokenKind::Punctuation, ";"),
            ],
        }],
    };
