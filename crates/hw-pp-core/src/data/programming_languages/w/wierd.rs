//! Hello-world snippet for `ProgrammingLanguageName::Wierd`.
//!
//! Imported from leachim6/hello-world (`w/Wierd.w` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Wierd => "Wierd",
    versions: [Wierd => "Wierd"],
    extensions: [W => "w"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wierd,
        extensions: &[FileExtension::W],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wierd,
            syntax: r"-*******************           dlroW olleH
 *
 R*   *    *     *
 E*  **    **   *
 T*  **    **  *
 U ***   ***  ****
 R   *  *  *      *
 N  *    *  ********
    *  ****
 0   **",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"-*******************           dlroW olleH
 *
 R*   *    *     *
 E*  **    **   *
 T*  **    **  *
 U ***   ***  ****
 R   *  *  *      *
 N  *    *  ********
    *  ****
 0   **",
            )],
        }],
    };
