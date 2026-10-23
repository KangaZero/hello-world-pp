//! Hello-world greetings for `HumanLanguageName::Solresol`.
//!
//! Sources:
//! - <https://omniglot.com/language/phrases/solresol.htm> (`simi`, "hello")
//! - <https://wiki.xxiivv.com/site/solresol.html> (Sudre's dictionary: `dodore`, "zero, globe, earth")
//! - <https://en.wikipedia.org/wiki/Solresol> (syllables and Sudre/Gajewski/Modern versions)

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Auxiliary, Constructed},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Solresol => "Solresol",
    dialects: [SudreSolresol => "Sudre Solresol"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Solresol,
    categories: &[Auxiliary, Constructed],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::SudreSolresol,
        greetings: &[
            // simi "hello" + dodore "globe, earth" (Sudre's dictionary has no "world");
            // composed from attested words, the phrase itself is unattested.
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "si-mi do-do-re",
            },
        ],
    }],
};
