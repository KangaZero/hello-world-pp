//! Hello-world snippet for `ProgrammingLanguageName::Pearl`.
//!
//! Imported from the Hello World Collection (`files/PEARL.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/PEARL.txt>
//! - <http://helloworldcollection.de/#PEARL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Pearl => "PEARL",
    versions: [Pearl => "PEARL"],
    extensions: [],
}

const PEARL: &str = concat!(
    r"MODULE (HELLO);
/* ",
    output_for_hello_world!(),
    r" in PEARL (Process and Experiment Automation Realtime Language) */
  SYSTEM;
   TERMINAL:DIS<->SDVLS(2);
  PROBLEM;
   SPC TERMINAL DATION INOUT
         ALPHIC DIM(,) TFU MAX
         FORWARD CONTROL (ALL);
   MAIN:TASK;
     DCL TEXT INV CHAR(30)
          INIT('",
    output_for_hello_world!(),
    r"');
    OPEN TERMINAL;
    PUT TEXT TO TERMINAL;
    CLOSE TERMINAL;
   END;
MODEND;"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pearl,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pearl,
            syntax: PEARL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PEARL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"MODULE (HELLO);
/* ",
                            output_for_hello_world!(),
                            r" in PEARL (Process and Experiment Automation Realtime Language) */
  SYSTEM;
   TERMINAL:DIS<->SDVLS(2);
  PROBLEM;
   SPC TERMINAL DATION INOUT
         ALPHIC DIM(,) TFU MAX
         FORWARD CONTROL (ALL);
   MAIN:TASK;
     DCL TEXT INV CHAR(30)
          INIT('",
                            output_for_hello_world!(),
                            r"');
    OPEN TERMINAL;
    PUT TEXT TO TERMINAL;
    CLOSE TERMINAL;
   END;
MODEND;"
                        ),
                    )],
                )
            },
        }],
    };
