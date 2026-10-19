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
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Asdf => "asdf",
    versions: [Asdf => "asdf"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Asdf,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Asdf,
            syntax: r"asaasaasssasaasssasaaaasssasaasssasaaaaaaaasssasaaaasssasaaaasssasaasssasaasss
asaasssasaasssasaaaasssasaasssasaaaasssasaaaaaasssasaaaasssasaasssasaaaasssasa
aaaaasssasaaaasssasaasssasaaaaaaaasssasaaaasssasaasssasaasssasaaaasssasaaaaaaa
asssasaasssasaaaaaaaaaaaasssasaaaaaasssasaasssasaaaaaasssasaasssasaaaasssasaas
ssasaaaaaaaasssasaasssasaaaaaasssasaaaasssasaasssasaaaasssasaaaasssasaasssasaa
aasssasaaaaaasssasaaaasssasaaaasssasaasssasaaaaaaaasssasaasssasaaaaaaaasssasaa",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"asaasaasssasaasssasaaaasssasaasssasaaaaaaaasssasaaaasssasaaaasssasaasssasaasss
asaasssasaasssasaaaasssasaasssasaaaasssasaaaaaasssasaaaasssasaasssasaaaasssasa
aaaaasssasaaaasssasaasssasaaaaaaaasssasaaaasssasaasssasaasssasaaaasssasaaaaaaa
asssasaasssasaaaaaaaaaaaasssasaaaaaasssasaasssasaaaaaasssasaasssasaaaasssasaas
ssasaaaaaaaasssasaasssasaaaaaasssasaaaasssasaasssasaaaasssasaaaasssasaasssasaa
aasssasaaaaaasssasaaaasssasaaaasssasaasssasaaaaaaaasssasaasssasaaaaaaaasssasaa",
            )],
        }],
    };
