//! Hello-world snippet for `ProgrammingLanguageName::NeverGonnaFuckYouUp`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Never_Gonna_Fuck_You_Up>
//! - <https://esolangs.org/wiki/Never_Gonna_Fuck_You_Up>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: NeverGonnaFuckYouUp => "Never Gonna Fuck You Up",
    versions: [NeverGonnaFuckYouUp => "Never Gonna Fuck You Up"],
    extensions: [],
}

const NEVER_GONNA_FUCK_YOU_UP: &str = r"Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
We are no strangers to love
Never gonna run around and desert you
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
We are no strangers to love
Never gonna run around and desert you
Never gonna give you up
Never gonna give you up
Never gonna run around and desert you
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna run around and desert you
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna run around and desert you
Never gonna give you up
Never gonna make you cry
Never gonna make you cry
Never gonna make you cry
Never gonna make you cry
Never gonna let you down
You know the rules and so do I (do I)
Never gonna run around and desert you
Never gonna give you up
Never gonna run around and desert you
Never gonna give you up
Never gonna run around and desert you
Never gonna let you down
Never gonna run around and desert you
Never gonna run around and desert you
Never gonna give you up
We are no strangers to love
Never gonna make you cry
You know the rules and so do I (do I)
Never gonna make you cry
Never gonna let you down
You know the rules and so do I (do I)
Never gonna run around and desert you
Never gonna run around and desert you
Never gonna say goodbye
Never gonna run around and desert you
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna say goodbye
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna say goodbye
Never gonna say goodbye
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna say goodbye
Never gonna run around and desert you
Never gonna run around and desert you
Never gonna say goodbye
Never gonna make you cry
Never gonna let you down
Never gonna say goodbye
Never gonna make you cry
Never gonna say goodbye
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna say goodbye
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna say goodbye
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna say goodbye
Never gonna run around and desert you
Never gonna run around and desert you
Never gonna give you up
Never gonna say goodbye
Never gonna run around and desert you
Never gonna give you up
Never gonna give you up
Never gonna say goodbye";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::NeverGonnaFuckYouUp,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::NeverGonnaFuckYouUp,
            syntax: NEVER_GONNA_FUCK_YOU_UP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NEVER_GONNA_FUCK_YOU_UP,
                    [(
                        TokenKind::Plain,
                        r"Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
We are no strangers to love
Never gonna run around and desert you
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
We are no strangers to love
Never gonna run around and desert you
Never gonna give you up
Never gonna give you up
Never gonna run around and desert you
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna run around and desert you
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna run around and desert you
Never gonna give you up
Never gonna make you cry
Never gonna make you cry
Never gonna make you cry
Never gonna make you cry
Never gonna let you down
You know the rules and so do I (do I)
Never gonna run around and desert you
Never gonna give you up
Never gonna run around and desert you
Never gonna give you up
Never gonna run around and desert you
Never gonna let you down
Never gonna run around and desert you
Never gonna run around and desert you
Never gonna give you up
We are no strangers to love
Never gonna make you cry
You know the rules and so do I (do I)
Never gonna make you cry
Never gonna let you down
You know the rules and so do I (do I)
Never gonna run around and desert you
Never gonna run around and desert you
Never gonna say goodbye
Never gonna run around and desert you
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna say goodbye
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna say goodbye
Never gonna say goodbye
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna say goodbye
Never gonna run around and desert you
Never gonna run around and desert you
Never gonna say goodbye
Never gonna make you cry
Never gonna let you down
Never gonna say goodbye
Never gonna make you cry
Never gonna say goodbye
Never gonna give you up
Never gonna give you up
Never gonna give you up
Never gonna say goodbye
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna say goodbye
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna let you down
Never gonna say goodbye
Never gonna run around and desert you
Never gonna run around and desert you
Never gonna give you up
Never gonna say goodbye
Never gonna run around and desert you
Never gonna give you up
Never gonna give you up
Never gonna say goodbye",
                    )],
                )
            },
        }],
    };
