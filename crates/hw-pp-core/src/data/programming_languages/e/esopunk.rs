//! Hello-world snippet for `ProgrammingLanguageName::Esopunk`.
//!
//! Imported from leachim6/hello-world (`e/ESOPUNK.exa` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/ESOPUNK.exa>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Esopunk => "ESOPUNK",
    versions: [Esopunk => "ESOPUNK"],
    extensions: [Exa => "exa"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Esopunk,
        extensions: &[FileExtension::Exa],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Esopunk,
            syntax: r"COPY 72 #STDO
COPY 101 #STDO
COPY 108 #STDO
COPY 108 #STDO
COPY 111 #STDO
COPY 32 #STDO
COPY 119 #STDO
COPY 111 #STDO
COPY 114 #STDO
COPY 108 #STDO
COPY 100 #STDO",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"COPY 72 #STDO
COPY 101 #STDO
COPY 108 #STDO
COPY 108 #STDO
COPY 111 #STDO
COPY 32 #STDO
COPY 119 #STDO
COPY 111 #STDO
COPY 114 #STDO
COPY 108 #STDO
COPY 100 #STDO",
            )],
        }],
    };
