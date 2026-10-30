//! Hello-world snippet for `ProgrammingLanguageName::Lnusp`.
//!
//! Imported from leachim6/hello-world (`l/LNUSP.lnusp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/LNUSP.lnusp>
//! - <https://esolangs.org/wiki/LNUSP> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{CellBased, TwoDimensional};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Lnusp => "LNUSP",
    versions: [Lnusp => "LNUSP"],
    extensions: [Lnusp => "lnusp"],
}

const LNUSP: &str = concat!(
    r"--- ",
    output_for_hello_world!(),
    r"
---
000        .INPUT          .OUTPUT          .STOP
001 .                      .                .
001  .                     .                .
001   .                    .                .
001    . !.!               .                .
001     .   !              .                .
001    ! .  !              .                .
001    .  !!               .                .
001    *                   .                .
072    + H                 .                .
001    *                   .                .
101    + e                 .                .
001    *                   .                .
108    + l                 .                .
001    *                   .                .
108    + l                 .                .
001    *                   .                .
111    + o                 .                .
001    *                   .                .
032    + (sp)              .                .
001    *                   .                .
087    + W                 .                .
001    *                   .                .
111    + o                 .                .
001    *                   .                .
114    + r                 .                .
001    *                   .                .
108    + l                 .                .
001    *                   .                .
100    + d                 .                .
001    *                   .                .
033    + !                 .                .
001    *                   .                .
013    + (cr)              . -------------- .
001    *                   .                .
001    !                   .                .
001     .                  .                .
001      .        !!       .                .
001       . !.!  !  !      .    !......!    .
001        .   ! .  !   ?......!.....?  !   .
001       ! .  ! . @   ?   .   .??    ? .   .
001       .  !!  ./.   .   .   ?  ?   . .   .
016       .      . * --.-- . --.  .---.-.-- .
001       .      ! .   ?   .   *  ?   . !   .
001       .       !.....?..@.....?.....!....@
001       .        .           ?      ?    .
001       !        !           ..    .    .
001        !......!            ! ?..?    .
001                             !.......!"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lnusp,
        extensions: &[FileExtension::Lnusp],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased, TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lnusp,
            syntax: LNUSP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LNUSP,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"--- ",
                            output_for_hello_world!(),
                            r"
---
000        .INPUT          .OUTPUT          .STOP
001 .                      .                .
001  .                     .                .
001   .                    .                .
001    . !.!               .                .
001     .   !              .                .
001    ! .  !              .                .
001    .  !!               .                .
001    *                   .                .
072    + H                 .                .
001    *                   .                .
101    + e                 .                .
001    *                   .                .
108    + l                 .                .
001    *                   .                .
108    + l                 .                .
001    *                   .                .
111    + o                 .                .
001    *                   .                .
032    + (sp)              .                .
001    *                   .                .
087    + W                 .                .
001    *                   .                .
111    + o                 .                .
001    *                   .                .
114    + r                 .                .
001    *                   .                .
108    + l                 .                .
001    *                   .                .
100    + d                 .                .
001    *                   .                .
033    + !                 .                .
001    *                   .                .
013    + (cr)              . -------------- .
001    *                   .                .
001    !                   .                .
001     .                  .                .
001      .        !!       .                .
001       . !.!  !  !      .    !......!    .
001        .   ! .  !   ?......!.....?  !   .
001       ! .  ! . @   ?   .   .??    ? .   .
001       .  !!  ./.   .   .   ?  ?   . .   .
016       .      . * --.-- . --.  .---.-.-- .
001       .      ! .   ?   .   *  ?   . !   .
001       .       !.....?..@.....?.....!....@
001       .        .           ?      ?    .
001       !        !           ..    .    .
001        !......!            ! ?..?    .
001                             !.......!"
                        ),
                    )],
                )
            },
        }],
    };
