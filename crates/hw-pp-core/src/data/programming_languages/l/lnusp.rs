//! Hello-world snippet for `ProgrammingLanguageName::Lnusp`.
//!
//! Imported from leachim6/hello-world (`l/LNUSP.lnusp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Lnusp => "LNUSP",
    versions: [Lnusp => "LNUSP"],
    extensions: [Lnusp => "lnusp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lnusp,
        extensions: &[FileExtension::Lnusp],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lnusp,
            syntax: concat!(
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
            tokenized_syntax: &[(
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
        }],
    };
