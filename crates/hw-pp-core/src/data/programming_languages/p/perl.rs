//! Hello-world snippets for `ProgrammingLanguageName::Perl`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Perl => "Perl",
    versions: [Perl5 => "Perl 5"],
    extensions: [Pl => "pl", Pm => "pm"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Perl,
        extensions: &[FileExtension::Pl, FileExtension::Pm],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Perl5,
            syntax: concat!(r#"print ""#, output_for_hello_world!(), r#"\n";"#),
            tokenized_syntax: &[
                (TokenKind::Function, "print"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), r#"\n""#),
                ),
                (TokenKind::Plain, ";"),
            ],
        }],
    };
