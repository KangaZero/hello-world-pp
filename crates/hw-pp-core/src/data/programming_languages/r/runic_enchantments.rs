//! Hello-world snippet for `ProgrammingLanguageName::RunicEnchantments`.
//!
//! `>` starts the IP moving right, `"..."` pushes the string, `$` prints the top of the
//! stack and `;` terminates; written from the documented commands (the repository is
//! unlicensed, so nothing is copied from it).
//!
//! Sources:
//! - <https://github.com/Draco18s/RunicEnchantments/tree/Console#hello-world>

use crate::data::macros::language_names;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

language_names! {
    language: RunicEnchantments => "Runic Enchantments",
    versions: [RunicEnchantments => "Runic Enchantments"],
    extensions: [],
}

const RUNIC_ENCHANTMENTS: &str = r#">"Hello, World!"$;"#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::RunicEnchantments,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::RunicEnchantments,
            syntax: RUNIC_ENCHANTMENTS,
            tokenized_syntax: &const {
                checked_token_lengths(RUNIC_ENCHANTMENTS, [(TokenKind::Plain, RUNIC_ENCHANTMENTS)])
            },
        }],
    };
