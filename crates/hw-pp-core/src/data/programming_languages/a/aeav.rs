//! Hello-world snippet for `ProgrammingLanguageName::Aeav`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#AEAV>
//! - <https://esolangs.org/wiki/AEAV>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Aeav => "AEAV",
    versions: [Aeav => "AEAV"],
    extensions: [],
}

const AEAV: &str = r#"helloworld = "Hello, "
helloworld = $$ "World!"
out helloworld"#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Aeav,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Aeav,
            syntax: AEAV,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AEAV,
                    [(
                        TokenKind::Plain,
                        r#"helloworld = "Hello, "
helloworld = $$ "World!"
out helloworld"#,
                    )],
                )
            },
        }],
    };
