//! Hello-world snippet for `ProgrammingLanguageName::Wierd`.
//!
//! Imported from leachim6/hello-world (`w/Wierd.w` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/w/Wierd.w>
//! - <https://esolangs.org/wiki/Wierd> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{StackBased, TwoDimensional};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Wierd => "Wierd",
    versions: [Wierd => "Wierd"],
    extensions: [W => "w"],
}

const WIERD: &str = r"-*******************           dlroW olleH
 *
 R*   *    *     *
 E*  **    **   *
 T*  **    **  *
 U ***   ***  ****
 R   *  *  *      *
 N  *    *  ********
    *  ****
 0   **";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wierd,
        extensions: &[FileExtension::W],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased, TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wierd,
            syntax: WIERD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    WIERD,
                    [(
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
                )
            },
        }],
    };
