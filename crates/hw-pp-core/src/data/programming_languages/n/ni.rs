//! Hello-world snippet for `ProgrammingLanguageName::Ni`.
//!
//! Imported from leachim6/hello-world (`n/Ni.nic` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/Ni.nic>
//! - <https://esolangs.org/wiki/Ni> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{CellBased, Imperative};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ni => "Ni",
    versions: [Ni => "Ni"],
    extensions: [Nic => "nic"],
}

const NI: &str = r"#48!#65!#6c!#6c!#6f!#20!#57!#6f!#72!#6c!#64!";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ni,
        extensions: &[FileExtension::Nic],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ni,
            syntax: NI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NI,
                    [(
                        TokenKind::Plain,
                        r"#48!#65!#6c!#6c!#6f!#20!#57!#6f!#72!#6c!#64!",
                    )],
                )
            },
        }],
    };
