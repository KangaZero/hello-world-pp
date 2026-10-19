//! Hello-world snippet for `ProgrammingLanguageName::Il`.
//!
//! Imported from leachim6/hello-world (`i/Il.il` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/Il.il>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Il => "Il",
    versions: [Il => "Il"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Il,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Il,
            syntax: concat!(
                r#"
// assembly references
.assembly extern mscorlib
{
  .publickeytoken = (B7 7A 5C 56 19 34 E0 89 )
  .ver 4:0:0:0
}

// assembly manifest
.assembly ilhw
{
  .custom instance void [mscorlib]System.Runtime.Versioning.TargetFrameworkAttribute::.ctor(string)
    = ( 01 00 1C 2E 4E 45 54 46 72 61 6D 65 77 6F 72 6B
        2C 56 65 72 73 69 6F 6E 3D 76 34 2E 35 2E 33 01
        00 54 0E 14 46 72 61 6D 65 77 6F 72 6B 44 69 73
        70 6C 61 79 4E 61 6D 65 14 2E 4E 45 54 20 46 72
        61 6D 65 77 6F 72 6B 20 34 2E 35 2E 33 )


  .hash algorithm 0x00008004
  .ver 0:0:0:0
}

.module         ilhw.dll
.imagebase      0x00400000
.file alignment 0x00000200
.stackreserve   0x00100000
.subsystem      0x0003
.corflags       0x00000001


.class public abstract auto ansi sealed ILHW.Program
  extends [mscorlib]System.Object // nope, not implicit P:
{

  .method public hidebysig static void Main(class [mscorlib]System.String[] args) cil managed
  {
    .entrypoint // this method is the entry point of the program
    .maxstack 1 // the maximum amount of objects on the stack is 1.

    ldstr ""#,
                output_for_hello_world!(),
                r#""
    call void [mscorlib]System.Console::WriteLine(string)

    ret
  }

}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"
// assembly references
.assembly extern mscorlib
{
  .publickeytoken = (B7 7A 5C 56 19 34 E0 89 )
  .ver 4:0:0:0
}

// assembly manifest
.assembly ilhw
{
  .custom instance void [mscorlib]System.Runtime.Versioning.TargetFrameworkAttribute::.ctor(string)
    = ( 01 00 1C 2E 4E 45 54 46 72 61 6D 65 77 6F 72 6B
        2C 56 65 72 73 69 6F 6E 3D 76 34 2E 35 2E 33 01
        00 54 0E 14 46 72 61 6D 65 77 6F 72 6B 44 69 73
        70 6C 61 79 4E 61 6D 65 14 2E 4E 45 54 20 46 72
        61 6D 65 77 6F 72 6B 20 34 2E 35 2E 33 )


  .hash algorithm 0x00008004
  .ver 0:0:0:0
}

.module         ilhw.dll
.imagebase      0x00400000
.file alignment 0x00000200
.stackreserve   0x00100000
.subsystem      0x0003
.corflags       0x00000001


.class public abstract auto ansi sealed ILHW.Program
  extends [mscorlib]System.Object // nope, not implicit P:
{

  .method public hidebysig static void Main(class [mscorlib]System.String[] args) cil managed
  {
    .entrypoint // this method is the entry point of the program
    .maxstack 1 // the maximum amount of objects on the stack is 1.

    ldstr ""#,
                    output_for_hello_world!(),
                    r#""
    call void [mscorlib]System.Console::WriteLine(string)

    ret
  }

}"#
                ),
            )],
        }],
    };
