//! Hello-world snippet for `ProgrammingLanguageName::Rando`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#RANDo>
//! - <https://esolangs.org/wiki/RANDo>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Rando => "RANDo",
    versions: [Rando => "RANDo"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rando,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rando,
            syntax: r"rrrrrrrrrrnrrrrrrrrrrnrrrrrrrrrrnrrrrrrrrrrndo
rrrrrrrrrrnrrrrrrrrrrnrrrrrrrrrrnrrrnrnanananrnando
rrrrrrrrrrnrrrrrrrrrrnrrrrrrrrrrrrrrrnaaaaaanaaaaaando
rrrrrrrrrrrrrrrrrrrrnrrrrrrrrrrnrrrrrnrrnrndo
rrrrrrrrrrrrrrrrrrrrnrrrrrrrrrrnrrrrrnrrnrndo
ond
rrrrrrrrrrrrrrrrrrrrrrrrrnrrrrrrrrrrnrrrrrnrrrrnrnaananando
rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrnrrrrrnrnrnaanananrnaanananando
rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrnrrrrrrrrrrnrnananrnrnando
rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrnaaaaanaaaaanrnrnando
rrrrrrrrrrrrrrrrrrrrnrrrrrrrnrnrnanananando",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"rrrrrrrrrrnrrrrrrrrrrnrrrrrrrrrrnrrrrrrrrrrndo
rrrrrrrrrrnrrrrrrrrrrnrrrrrrrrrrnrrrnrnanananrnando
rrrrrrrrrrnrrrrrrrrrrnrrrrrrrrrrrrrrrnaaaaaanaaaaaando
rrrrrrrrrrrrrrrrrrrrnrrrrrrrrrrnrrrrrnrrnrndo
rrrrrrrrrrrrrrrrrrrrnrrrrrrrrrrnrrrrrnrrnrndo
ond
rrrrrrrrrrrrrrrrrrrrrrrrrnrrrrrrrrrrnrrrrrnrrrrnrnaananando
rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrnrrrrrnrnrnaanananrnaanananando
rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrnrrrrrrrrrrnrnananrnrnando
rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrnaaaaanaaaaanrnrnando
rrrrrrrrrrrrrrrrrrrrnrrrrrrrnrnrnanananando",
            )],
        }],
    };
