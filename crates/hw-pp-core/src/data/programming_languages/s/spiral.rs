//! Hello-world snippet for `ProgrammingLanguageName::Spiral`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Spiral>
//! - <https://esolangs.org/wiki/Spiral>

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, TwoDimensional};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Spiral => "Spiral",
    versions: [Spiral => "Spiral"],
    extensions: [],
}

const SPIRAL: &str = r"e0v ***   *eXlv**   *lX      *2X       **oXi
v * * *   *     *   * 2      * o      **   v*
* * * *   * *****   * v      * v      * *iX *
* * * *   * *       * ^      v *      * * w *
* *** *   * *****   * v      * *      * * v *
*     *   *     ^   * ^      * *      * * * *
* *** *   * ****v   * v      * *      v * * *
* * * *   * *       * *      * *      ^ * * *
* * * *   * *****   * *****  * *****  * *** *
* * * *   *     *   *     *  *     *  **   **
*** ***   *******   *****v^  *******   *****


*wX ***    **3Xp    *rX4..   d5*      qd**  
* 3 * *   **   v^   *    ..  * *      *  ***
* v * ^   * #pX v   * ..  .  * *      *    **
* *** v   * # r #   * ..  .  * *      * !q* *
* * * *   * # v #   * 54 ..  * *      * * * *
* * * *   * # * #   *   @X   * *      * * * *
* * * *   * # * #   *   v    * *      * * * *
* * * *   * # * #   * * **   * *      * * * *
*  *  *   * # * #   * ** **  * *      * *** *
*  *  *   * #v* ^   * *** *  * *****  *    **
** * **   **   *v   * * * *  *     *  *  ***
 *****     *v^**    *** ***  *******  **** ";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Spiral,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Spiral,
            syntax: SPIRAL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SPIRAL,
                    [(
                        TokenKind::Plain,
                        r"e0v ***   *eXlv**   *lX      *2X       **oXi
v * * *   *     *   * 2      * o      **   v*
* * * *   * *****   * v      * v      * *iX *
* * * *   * *       * ^      v *      * * w *
* *** *   * *****   * v      * *      * * v *
*     *   *     ^   * ^      * *      * * * *
* *** *   * ****v   * v      * *      v * * *
* * * *   * *       * *      * *      ^ * * *
* * * *   * *****   * *****  * *****  * *** *
* * * *   *     *   *     *  *     *  **   **
*** ***   *******   *****v^  *******   *****


*wX ***    **3Xp    *rX4..   d5*      qd**  
* 3 * *   **   v^   *    ..  * *      *  ***
* v * ^   * #pX v   * ..  .  * *      *    **
* *** v   * # r #   * ..  .  * *      * !q* *
* * * *   * # v #   * 54 ..  * *      * * * *
* * * *   * # * #   *   @X   * *      * * * *
* * * *   * # * #   *   v    * *      * * * *
* * * *   * # * #   * * **   * *      * * * *
*  *  *   * # * #   * ** **  * *      * *** *
*  *  *   * #v* ^   * *** *  * *****  *    **
** * **   **   *v   * * * *  *     *  *  ***
 *****     *v^**    *** ***  *******  **** ",
                    )],
                )
            },
        }],
    };
