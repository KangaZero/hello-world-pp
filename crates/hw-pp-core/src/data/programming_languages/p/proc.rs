//! Hello-world snippet for `ProgrammingLanguageName::Proc`.
//!
//! Imported from the Hello World Collection (`files/ProC.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ProC.txt>
//! - <http://helloworldcollection.de/#ProC>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::{C, Cpp},
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Proc => "ProC",
    versions: [Proc => "ProC"],
    extensions: [],
}

const PROC: &str = concat!(
    r"/* ",
    output_for_hello_world!(),
    r#" in Pro*C, the Oracle's embedded SQL environment */

#include <stdio.h>

EXEC SQL INCLUDE SQLCA;

int main() {
   char hello[15];
   char *user = "the_user";
   char *password = "the_password";
   char *sid = "the_sid";

   EXEC SQL CONNECT :user IDENTIFIED BY :password USING :sid;

   EXEC SQL
      SELECT '"#,
    output_for_hello_world!(),
    r#"' INTO :hello
      FROM DUAL;

   printf("%s\n", hello);

   EXEC SQL COMMIT RELEASE;
 
   return 0;
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Proc,
        extensions: &[],
        categories: &[],
        families: &[C, Cpp],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Proc,
            syntax: PROC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PROC,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"/* ",
                            output_for_hello_world!(),
                            r#" in Pro*C, the Oracle's embedded SQL environment */

#include <stdio.h>

EXEC SQL INCLUDE SQLCA;

int main() {
   char hello[15];
   char *user = "the_user";
   char *password = "the_password";
   char *sid = "the_sid";

   EXEC SQL CONNECT :user IDENTIFIED BY :password USING :sid;

   EXEC SQL
      SELECT '"#,
                            output_for_hello_world!(),
                            r#"' INTO :hello
      FROM DUAL;

   printf("%s\n", hello);

   EXEC SQL COMMIT RELEASE;
 
   return 0;
}"#
                        ),
                    )],
                )
            },
        }],
    };
