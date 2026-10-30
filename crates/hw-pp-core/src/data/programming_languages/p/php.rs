//! Hello-world snippet for `ProgrammingLanguageName::Php`.
//!
//! Sources:
//! - <https://www.php.net/manual/en/tutorial.firstpage.php>
//! - <https://www.php.net/manual/en/function.echo.php>
//! - <https://www.wikidata.org/wiki/Q59> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented, Procedural, Reflective,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::C, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Php => "PHP",
    versions: [Php8 => "PHP 8"],
    extensions: [Php => "php"],
}

const PHP8: &str = concat!(
    r#"<?php
echo ""#,
    output_for_hello_world!(),
    r#"\n";"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Php,
        extensions: &[FileExtension::Php],
        categories: &[Scripting],
        families: &[C],
        paradigms: &[
            Imperative,
            Procedural,
            ObjectOriented,
            Functional,
            Reflective,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Php8,
            syntax: PHP8,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PHP8,
                    [
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
                )
            },
        }],
    };
