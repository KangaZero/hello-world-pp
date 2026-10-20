//! Hello-world snippet for `ProgrammingLanguageName::Ti59`.
//!
//! Imported from the Hello World Collection (`files/TI-59.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/TI-59.htm>
//! - <http://helloworldcollection.de/#TI-59>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Ti59 => "TI-59",
    versions: [Ti59 => "TI-59"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ti59,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ti59,
            syntax: r#"      Code        Comment

      LBL A       Start of program: label A
      OP 00       Clear the four print registers
      23          "H"
      17          "E"
      OP 02       Write into print register 2
      27          "L"
      27          "L"
      32          "O"
      00          " "
      43          "W"
      OP 03       Write into print register 3
      32          "O"
      35          "R"
      27          "L"
      16          "D"
      73          "!"
      OP 04       Write into print register 4
      OP 05       Start printing
      ADV         Line feed (optional)
      R/S         End program"#,
            tokenized_syntax: &[(
                TokenKind::Plain,
                r#"      Code        Comment

      LBL A       Start of program: label A
      OP 00       Clear the four print registers
      23          "H"
      17          "E"
      OP 02       Write into print register 2
      27          "L"
      27          "L"
      32          "O"
      00          " "
      43          "W"
      OP 03       Write into print register 3
      32          "O"
      35          "R"
      27          "L"
      16          "D"
      73          "!"
      OP 04       Write into print register 4
      OP 05       Start printing
      ADV         Line feed (optional)
      R/S         End program"#,
            )],
        }],
    };
