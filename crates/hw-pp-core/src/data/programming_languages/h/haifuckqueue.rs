//! Hello-world snippet for `ProgrammingLanguageName::Haifuckqueue`.
//!
//! Imported from leachim6/hello-world (`h/Haifuckqueue` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Haifuckqueue>
//! - <https://esolangs.org/wiki/Haifuckqueue> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Haifuckqueue => "Haifuckqueue",
    versions: [Haifuckqueue => "Haifuckqueue"],
    extensions: [],
}

const HAIFUCKQUEUE: &str = r"{Hel}
$3{lo }
$3{W}
$1{o}
$1{rld}
$3{ }
$1|10
$1ooooo
ooooo ";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Haifuckqueue,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Haifuckqueue,
            syntax: HAIFUCKQUEUE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HAIFUCKQUEUE,
                    [(
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
                )
            },
        }],
    };
