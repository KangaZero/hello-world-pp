//! Hello-world snippet for `ProgrammingLanguageName::U062bU0644U0627U062bU064a`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#%D8%AB%D9%84%D8%A7%D8%AB%D9%8A>
//! - <https://esolangs.org/wiki/Triliteral>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: U062bU0644U0627U062bU064a => "ثلاثي",
    versions: [U062bU0644U0627U062bU064a => "ثلاثي"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U062bU0644U0627U062bU064a,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U062bU0644U0627U062bU064a,
            syntax: r"ﺎﻜﻄﺑ  ﻲﻃﺍ  ﻲﻜﻄﺑ ﻼﻃﺭ ﻝﻮﻃﺍﺭ ﻲﻠﻃﺭ ﺱﺎﻜﺿ ﻮﺳﺎﻛﺎﺿ ﺎﻜﻠﻃ ﻕ ﻲﻜﻠﻃ ﺎﺳﻮﻜﺿ ﺎﺴﻴﻜﺿ ﻲﻠﻴﻃﺍﺭ ﻙﺎﻃﺎﺑ ﺎﺴﻤﺣ ﺍ ﻲﺴﻤﺣ ﺎﻛﻮﻃﺎﺑ ﻮﺴﻴﻜﺿ ﻊﺑ ﻕﺍ ﻖﺣ ﻖﺣ ﻖﻳﺍ ﻞﺑ ﻖﻴﻃ ﻖﻳﺍ ﻖﻳﺩ ﻖﺣ ﻕ ﻞﺟ ﻱ",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"ﺎﻜﻄﺑ  ﻲﻃﺍ  ﻲﻜﻄﺑ ﻼﻃﺭ ﻝﻮﻃﺍﺭ ﻲﻠﻃﺭ ﺱﺎﻜﺿ ﻮﺳﺎﻛﺎﺿ ﺎﻜﻠﻃ ﻕ ﻲﻜﻠﻃ ﺎﺳﻮﻜﺿ ﺎﺴﻴﻜﺿ ﻲﻠﻴﻃﺍﺭ ﻙﺎﻃﺎﺑ ﺎﺴﻤﺣ ﺍ ﻲﺴﻤﺣ ﺎﻛﻮﻃﺎﺑ ﻮﺴﻴﻜﺿ ﻊﺑ ﻕﺍ ﻖﺣ ﻖﺣ ﻖﻳﺍ ﻞﺑ ﻖﻴﻃ ﻖﻳﺍ ﻖﻳﺩ ﻖﺣ ﻕ ﻞﺟ ﻱ",
            )],
        }],
    };
