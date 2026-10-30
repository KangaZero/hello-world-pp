//! Hello-world greetings for `HumanLanguageName::Xoo`.
//!
//! Display name ǃXóõ (Taa); the endonym is Taa ǂaan. Typed as `taa` via the alias.
//!
//! Sources:
//! - <https://archive.org/details/xoodictionary_201910>
//! - <https://en.wiktionary.org/wiki/%C7%82kx%CA%BB%C3%BBm>
//! - <https://en.wikipedia.org/wiki/Taa_language>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Tuu, HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Xoo => "ǃXóõ" | "Taa",
    dialects: [EastXoo => "East ǃXóõ"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Xoo,
    categories: &[Natural],
    families: &[Tuu],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::EastXoo,
        greetings: &[
            // ǂgùm kòo "hullo there!" + ǂkxʼûm "sand, earth, the earth" (Traill 1994, pp. 135, 170,
            // 137; Lone Tree dialect); composed from attested words, the phrase itself is unattested.
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "ǂgùm kòo, ǂkxʼûm!",
            },
        ],
    }],
};
