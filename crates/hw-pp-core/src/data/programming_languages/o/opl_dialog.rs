//! Hello-world snippet for `ProgrammingLanguageName::OplDialog`.
//!
//! Imported from the Hello World Collection (`files/OPL.dialog` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/OPL.dialog>
//! - <http://helloworldcollection.de/#OPL.dialog>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: OplDialog => "OPL.dialog",
    versions: [OplDialog => "OPL.dialog"],
    extensions: [],
}

const OPL_DIALOG: &str = concat!(
    r"REM ",
    output_for_hello_world!(),
    r#" for OPL (Psion Organizer 3a)
REM More complex version with menues and dialog boxes

PROC HELLO:
  LOCAL M%
  DO
    REM Display menu bar
    mINIT
    mCARD "Sprache","Deutsch",%d,"English",%e
    mCARD "Extras","Beenden",%x,"Info",%i
    M%=MENU
    REM process choosen function
    IF M%=%d
      REM Display german dialog box
      REM with an ENTER button to continue
      dBOX:(" ","Hallo Welt"," ","weiter",13)
    ELSEIF M%=%e
      REM Display english dialog box
      REM with an ENTER button to continue
      dBOX:(" ",""#,
    output_for_hello_world!(),
    r#""," ","continue",13)
    ELSEIF M%=%i
      REM Display copyright information ;-)
      dBOX:("Info","(C) Klaus M&uuml;ller 0196","FrankfurtMain, Germany","",13)
    ENDIF
  UNTIL M%=%x
ENDP

PROC dBOX:(Z1$,Z2$,Z3$,Z4$,BUTTON%)
  dINIT Z1$
  dTEXT ""," ",0
  dTEXT "",Z2$",$102
  dTEXT "",Z3$,$202
  dBUTTONS Z4$,BUTTON%
  DIALOG
ENDP"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OplDialog,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OplDialog,
            syntax: OPL_DIALOG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OPL_DIALOG,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"REM ",
                            output_for_hello_world!(),
                            r#" for OPL (Psion Organizer 3a)
REM More complex version with menues and dialog boxes

PROC HELLO:
  LOCAL M%
  DO
    REM Display menu bar
    mINIT
    mCARD "Sprache","Deutsch",%d,"English",%e
    mCARD "Extras","Beenden",%x,"Info",%i
    M%=MENU
    REM process choosen function
    IF M%=%d
      REM Display german dialog box
      REM with an ENTER button to continue
      dBOX:(" ","Hallo Welt"," ","weiter",13)
    ELSEIF M%=%e
      REM Display english dialog box
      REM with an ENTER button to continue
      dBOX:(" ",""#,
                            output_for_hello_world!(),
                            r#""," ","continue",13)
    ELSEIF M%=%i
      REM Display copyright information ;-)
      dBOX:("Info","(C) Klaus M&uuml;ller 0196","FrankfurtMain, Germany","",13)
    ENDIF
  UNTIL M%=%x
ENDP

PROC dBOX:(Z1$,Z2$,Z3$,Z4$,BUTTON%)
  dINIT Z1$
  dTEXT ""," ",0
  dTEXT "",Z2$",$102
  dTEXT "",Z3$,$202
  dBUTTONS Z4$,BUTTON%
  DIALOG
ENDP"#
                        ),
                    )],
                )
            },
        }],
    };
