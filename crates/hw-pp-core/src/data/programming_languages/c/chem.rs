//! Hello-world snippet for `ProgrammingLanguageName::Chem`.
//!
//! Imported from leachim6/hello-world (`c/Chem` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Chem>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Chem => "Chem",
    versions: [Chem => "Chem"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Chem,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Chem,
            syntax: r"Chemicals needed:
72 g Ca
101 g Cl
Check to confirm you have the required chemicals before proceeding.
Chemicals are to be added to a small beaker.
Add the Ca.
Add the Cl.
Now find the following chemicals and mix together in a separate beaker:
108 ml water
108 g calcium
111 g sulfur
32 g carbon
87 ml mercury
111 g salt
114 g zinc
108 g tin
100 ml bromine
Going back to the original beaker,
Dump the contents of beaker 1 into this beaker.
Record your findings.",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"Chemicals needed:
72 g Ca
101 g Cl
Check to confirm you have the required chemicals before proceeding.
Chemicals are to be added to a small beaker.
Add the Ca.
Add the Cl.
Now find the following chemicals and mix together in a separate beaker:
108 ml water
108 g calcium
111 g sulfur
32 g carbon
87 ml mercury
111 g salt
114 g zinc
108 g tin
100 ml bromine
Going back to the original beaker,
Dump the contents of beaker 1 into this beaker.
Record your findings.",
            )],
        }],
    };
