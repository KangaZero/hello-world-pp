//! Hello-world snippet for `ProgrammingLanguageName::Thesquare`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#TheSquare>
//! - <https://esolangs.org/wiki/TheSquare>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Thesquare => "TheSquare",
    versions: [Thesquare => "TheSquare"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Thesquare,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Thesquare,
            syntax: r"v#############@
+:DDDDDDDDDDD:[
+#;;;;;;;;;;;#
+              
+ ;;
+  >v      ;
+ JJ+      >v
+>^-+      J-
+/--+      --
v{>[<;;    --
/- +  >v;  --
+\ +;JJ+>v --
++ +>^++J- --
++ +J++++-;--
++ >^>^++->^-
++     ++-Jv<
>^     >^-+ ;
#      v-<+>[ ^
#      - ;+
#      >-[^",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"v#############@
+:DDDDDDDDDDD:[
+#;;;;;;;;;;;#
+              
+ ;;
+  >v      ;
+ JJ+      >v
+>^-+      J-
+/--+      --
v{>[<;;    --
/- +  >v;  --
+\ +;JJ+>v --
++ +>^++J- --
++ +J++++-;--
++ >^>^++->^-
++     ++-Jv<
>^     >^-+ ;
#      v-<+>[ ^
#      - ;+
#      >-[^",
            )],
        }],
    };
