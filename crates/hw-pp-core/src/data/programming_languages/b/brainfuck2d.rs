//! Hello-world snippet for `ProgrammingLanguageName::Brainfuck2d`.
//!
//! Imported from leachim6/hello-world (`b/Brainfuck 2D.bf2d` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Brainfuck%202D.bf2d>
//! - <https://esolangs.org/wiki/Brainfuck_2D> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Brainfuck2d => "Brainfuck 2D",
    versions: [Brainfuck2d => "Brainfuck 2D"],
    extensions: [Bf2d => "bf2d"],
}

const BRAINFUCK2D: &str = concat!(
    r#"*
 *                                          *0**************
  *                                        *                *
   *                                      *                  *
    *9*******************                *          *         *7***************
                        *               *          **                         *
                        *              *          * *                         *
                        *             *          *  *                         *
                        *            *          *   *                         *
                        *           **********0*    *                         *
                       *                            **********                *
                      *                                     *                 *
                     *                                     *                  *
                    *44****************************       *                   *
                                                   *     *                    *
                                                    *   *                     *
      ***********0*                                  * 0                      *
     *            *                          *3***    *                       *
    *             *                         *     0                           *
   *              *                        2       *                          *
  *               *           *7***********         *                        *
 *               *           *                       *                      *
 0              *           *                         *                    *4***********
 *             *           *                           *                                *
 *            *           *                             *             *0****             *
 *           0           *                               *           *     *              *
 *          ********************************************************************************
 *                     *                                   *       *       *
 **********************                                     *     *        *
                                                       *     *0***         *
                                                       **                  *
                                                       * *                 *
                                                       *  *                *
                              *                        *********************
                             * *                            *
                            *   *                            *
                           *     *                            *8****************
                          *       *26****                                      *
                         *              *                                      *
                        *               *              *0******                *
                       *      *         *             *        *              *
                      *      **         *            *     *    *            *
                     *      * *         *           *     **     *          *4*******
                    *      *  *         *          *     * *      *                  *
                   *      *   *         *         *     *  *       *                  *
                  *      *    *         *        *****0*   *****************************
                 *      *     *         *                            *
                *      *      *        *                              *
               ******0*       *       *                                *
                              *      *                                  *92***********
                              *     *3******                                         *
                              *             *                                        *
                              *          *   *                                       *
                              *         **    *                                      *
                              *        * *     *        *0*****                     *
                              *       *  *      *      *     0                     *
                              *      *   *********    *     *                     *5****
                              *     *                *     *                            *
                              *     0      *3********     *                              *
                              *     *     *              *                                *
                              **************************************************************
                                    *   *              *
                                    *  *    *****8*   *
                                    * *     0    *   *    *
                                    **      *   *   *    **
                                    *       *  ***6*    * *                                  *
                     *                      *          *  *                                 *
                     **                     *0*********   *                                *
                     * *                                  *                               *
                     *  *                                 *                              *
                     *   *                                *                               *
                     *    *                               *                                *
                     **************************************                                 *
                            *                                                                0
                             *                                                 *91*************
                              *2222*****************************               *
                                                               *               *
                                                               *               *
                                                  *0**************************************   *
                                                  *            *               *        *   **
                                                  *            *               *       *   * *
                                                  *           *                *      *   *  *
                                                  *          *                 *     *   *   *
                                                  *         *                  *    *****    *
                                                  *        *31*******          *             *
                                                  *                  *         *             *
                                                  *                   *        ***************
                                                  *                    *
                                                  ***********************

/* the new BrainFuckTwoD Code
      ""#,
    output_for_hello_world!(),
    r#""
     (c) DuNe & oCaS          */
"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Brainfuck2d,
        extensions: &[FileExtension::Bf2d],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Brainfuck2d,
            syntax: BRAINFUCK2D,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BRAINFUCK2D,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"*
 *                                          *0**************
  *                                        *                *
   *                                      *                  *
    *9*******************                *          *         *7***************
                        *               *          **                         *
                        *              *          * *                         *
                        *             *          *  *                         *
                        *            *          *   *                         *
                        *           **********0*    *                         *
                       *                            **********                *
                      *                                     *                 *
                     *                                     *                  *
                    *44****************************       *                   *
                                                   *     *                    *
                                                    *   *                     *
      ***********0*                                  * 0                      *
     *            *                          *3***    *                       *
    *             *                         *     0                           *
   *              *                        2       *                          *
  *               *           *7***********         *                        *
 *               *           *                       *                      *
 0              *           *                         *                    *4***********
 *             *           *                           *                                *
 *            *           *                             *             *0****             *
 *           0           *                               *           *     *              *
 *          ********************************************************************************
 *                     *                                   *       *       *
 **********************                                     *     *        *
                                                       *     *0***         *
                                                       **                  *
                                                       * *                 *
                                                       *  *                *
                              *                        *********************
                             * *                            *
                            *   *                            *
                           *     *                            *8****************
                          *       *26****                                      *
                         *              *                                      *
                        *               *              *0******                *
                       *      *         *             *        *              *
                      *      **         *            *     *    *            *
                     *      * *         *           *     **     *          *4*******
                    *      *  *         *          *     * *      *                  *
                   *      *   *         *         *     *  *       *                  *
                  *      *    *         *        *****0*   *****************************
                 *      *     *         *                            *
                *      *      *        *                              *
               ******0*       *       *                                *
                              *      *                                  *92***********
                              *     *3******                                         *
                              *             *                                        *
                              *          *   *                                       *
                              *         **    *                                      *
                              *        * *     *        *0*****                     *
                              *       *  *      *      *     0                     *
                              *      *   *********    *     *                     *5****
                              *     *                *     *                            *
                              *     0      *3********     *                              *
                              *     *     *              *                                *
                              **************************************************************
                                    *   *              *
                                    *  *    *****8*   *
                                    * *     0    *   *    *
                                    **      *   *   *    **
                                    *       *  ***6*    * *                                  *
                     *                      *          *  *                                 *
                     **                     *0*********   *                                *
                     * *                                  *                               *
                     *  *                                 *                              *
                     *   *                                *                               *
                     *    *                               *                                *
                     **************************************                                 *
                            *                                                                0
                             *                                                 *91*************
                              *2222*****************************               *
                                                               *               *
                                                               *               *
                                                  *0**************************************   *
                                                  *            *               *        *   **
                                                  *            *               *       *   * *
                                                  *           *                *      *   *  *
                                                  *          *                 *     *   *   *
                                                  *         *                  *    *****    *
                                                  *        *31*******          *             *
                                                  *                  *         *             *
                                                  *                   *        ***************
                                                  *                    *
                                                  ***********************

/* the new BrainFuckTwoD Code
      ""#,
                            output_for_hello_world!(),
                            r#""
     (c) DuNe & oCaS          */
"#
                        ),
                    )],
                )
            },
        }],
    };
