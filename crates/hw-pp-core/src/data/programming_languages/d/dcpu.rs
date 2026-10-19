//! Hello-world snippet for `ProgrammingLanguageName::Dcpu`.
//!
//! Imported from leachim6/hello-world (`d/DCPU.dasm` at `a152253`),
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
    language: Dcpu => "DCPU",
    versions: [Dcpu => "DCPU"],
    extensions: [Dasm => "dasm"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dcpu,
        extensions: &[FileExtension::Dasm],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dcpu,
            syntax: concat!(
                r#"  set i, 0xf615
  set j, 0x7349
  hwn z
  sub z, 1

:device_detect_loop
  hwq z

  ife a, i
    ife b, j
      set pc, device_detect_ret

  sub z, 1
  ifa z, 0xffff
    set pc, device_detect_loop

:device_detect_ret
  set a, 0
  set b, [vidmem]
  hwi z

  set i, [vidmem]
  set j, string
  set c, 0xc

:memcpy_loop
  bor [j], 0xf000 ; OR with font style.
  sti [i], [j]
  sub c, 1
  ife c, 0
    sub pc, 1
  set pc, memcpy_loop

:display dat 0
:vidmem dat 0x8000
:string dat ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"  set i, 0xf615
  set j, 0x7349
  hwn z
  sub z, 1

:device_detect_loop
  hwq z

  ife a, i
    ife b, j
      set pc, device_detect_ret

  sub z, 1
  ifa z, 0xffff
    set pc, device_detect_loop

:device_detect_ret
  set a, 0
  set b, [vidmem]
  hwi z

  set i, [vidmem]
  set j, string
  set c, 0xc

:memcpy_loop
  bor [j], 0xf000 ; OR with font style.
  sti [i], [j]
  sub c, 1
  ife c, 0
    sub pc, 1
  set pc, memcpy_loop

:display dat 0
:vidmem dat 0x8000
:string dat ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
