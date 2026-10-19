//! Hello-world snippet for `ProgrammingLanguageName::Ni`.
//!
//! Imported from leachim6/hello-world (`n/Ni.nic` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Ni => "Ni",
    versions: [Ni => "Ni"],
    extensions: [Nic => "nic"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ni,
        extensions: &[FileExtension::Nic],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ni,
            syntax: r"#48!#65!#6c!#6c!#6f!#20!#57!#6f!#72!#6c!#64!",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"#48!#65!#6c!#6c!#6f!#20!#57!#6f!#72!#6c!#64!",
            )],
        }],
    };
