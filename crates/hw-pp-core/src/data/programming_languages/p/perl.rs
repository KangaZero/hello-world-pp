//! Hello-world snippet for `ProgrammingLanguageName::Perl`.
//!
//! Sources:
//! - <https://perldoc.perl.org/perlintro>
//! - <https://perldoc.perl.org/functions/print>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::{C, Perl},
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Perl => "Perl",
    versions: [Perl5 => "Perl 5"],
    extensions: [Pl => "pl", Pm => "pm"],
}

const PERL5: &str = concat!(r#"print ""#, output_for_hello_world!(), r#"\n";"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Perl,
        extensions: &[FileExtension::Pl, FileExtension::Pm],
        categories: &[Scripting],
        families: &[C, Perl],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Perl5,
            syntax: PERL5,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PERL5,
                    [
                        (TokenKind::Function, "print"),
                        (TokenKind::Plain, " "),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), r#"\n""#),
                        ),
                        (TokenKind::Plain, ";"),
                    ],
                )
            },
        }],
    };
