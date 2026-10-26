//! Hello-world snippet for `ProgrammingLanguageName::Asdf`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#asdf>
//! - <https://esolangs.org/wiki/Asdf>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Asdf => "asdf",
    versions: [Asdf => "asdf"],
    extensions: [],
}

const ASDF: &str = r"asaasaasssasaasssasaaaasssasaasssasaaaaaaaasssasaaaasssasaaaasssasaasssasaasss
asaasssasaasssasaaaasssasaasssasaaaasssasaaaaaasssasaaaasssasaasssasaaaasssasa
aaaaasssasaaaasssasaasssasaaaaaaaasssasaaaasssasaasssasaasssasaaaasssasaaaaaaa
asssasaasssasaaaaaaaaaaaasssasaaaaaasssasaasssasaaaaaasssasaasssasaaaasssasaas
ssasaaaaaaaasssasaasssasaaaaaasssasaaaasssasaasssasaaaasssasaaaasssasaasssasaa
aasssasaaaaaasssasaaaasssasaaaasssasaasssasaaaaaaaasssasaasssasaaaaaaaasssasaa";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Asdf,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Asdf,
            syntax: ASDF,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASDF,
                    [(
                        TokenKind::Plain,
                        r"asaasaasssasaasssasaaaasssasaasssasaaaaaaaasssasaaaasssasaaaasssasaasssasaasss
asaasssasaasssasaaaasssasaasssasaaaasssasaaaaaasssasaaaasssasaasssasaaaasssasa
aaaaasssasaaaasssasaasssasaaaaaaaasssasaaaasssasaasssasaasssasaaaasssasaaaaaaa
asssasaasssasaaaaaaaaaaaasssasaaaaaasssasaasssasaaaaaasssasaasssasaaaasssasaas
ssasaaaaaaaasssasaasssasaaaaaasssasaaaasssasaasssasaaaasssasaaaasssasaasssasaa
aasssasaaaaaasssasaaaasssasaaaasssasaasssasaaaaaaaasssasaasssasaaaaaaaasssasaa",
                    )],
                )
            },
        }],
    };
