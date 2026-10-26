//! Hello-world snippet for `ProgrammingLanguageName::NsBang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#NS!>
//! - <https://esolangs.org/wiki/NS!>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: NsBang => "NS!",
    versions: [NsBang => "NS!"],
    extensions: [],
}

const NS_BANG: &str = r"98*#AA*1+#AB*2-#AB*2-#AB*1+#4A*4+#48*#A9*3-#AB*1+#AB*4+#AB*2-#AA*#3A*3+#";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::NsBang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::NsBang,
            syntax: NS_BANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NS_BANG,
                    [(
                        TokenKind::Plain,
                        r"98*#AA*1+#AB*2-#AB*2-#AB*1+#4A*4+#48*#A9*3-#AB*1+#AB*4+#AB*2-#AA*#3A*3+#",
                    )],
                )
            },
        }],
    };
