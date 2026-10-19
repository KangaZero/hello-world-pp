//! Hello-world snippet for `ProgrammingLanguageName::Haifuckqueue`.
//!
//! Imported from leachim6/hello-world (`h/Haifuckqueue` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Haifuckqueue => "Haifuckqueue",
    versions: [Haifuckqueue => "Haifuckqueue"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Haifuckqueue,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Haifuckqueue,
            syntax: r"{Hel}
$3{lo }
$3{W}
$1{o}
$1{rld}
$3{ }
$1|10
$1ooooo
ooooo ",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"{Hel}
$3{lo }
$3{W}
$1{o}
$1{rld}
$3{ }
$1|10
$1ooooo
ooooo ",
            )],
        }],
    };
