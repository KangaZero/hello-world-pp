//! Hello-world snippet for `ProgrammingLanguageName::Recurse`.
//!
//! Imported from leachim6/hello-world (`r/Recurse.recurse` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Recurse.recurse>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Recurse => "Recurse",
    versions: [Recurse => "Recurse"],
    extensions: [Recurse => "recurse"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Recurse,
        extensions: &[FileExtension::Recurse],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Recurse,
            syntax: r" Main function
$###########v######################################
>..9n_3n{5Av# 'H' = 9*8       # 'w' = (2*8-1)*8-1 #
#vA3_[_A7_A<# 'e' = 'H'+3*8+5 # 'o' = 'W'-8       #
#>_[5n{4A_[v# 'l' = 'e'+7     # 'r' = 'o'+3       #
#vS1{n2[_n4<# 'l'             # 'l' = 'r'-6       #
#>n{1S_8S_3v# 'o' = 'l'+3     # 'd' = 'l'-8       #
#v[_S8_S6_A<# ',' = 5*8+ 4    # '!' = 4*8+1       #
#>4n{1A_[..!# ' ' = 4*8       #                   <
$###########^######################################

 Multiply by 8 (Bidirectional)
n####v#####
>{8}m#m}8{<
n####^#####

 Push on left stack and print (Bidirectional)
_##v###
>{!#!{<
_##^###

 Add (Bidirectional)
A##v###
>}a#a}<
A##^###

 Subtract (Bidirectional)
S##v###
>}s#s}<
S##^###",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r" Main function
$###########v######################################
>..9n_3n{5Av# 'H' = 9*8       # 'w' = (2*8-1)*8-1 #
#vA3_[_A7_A<# 'e' = 'H'+3*8+5 # 'o' = 'W'-8       #
#>_[5n{4A_[v# 'l' = 'e'+7     # 'r' = 'o'+3       #
#vS1{n2[_n4<# 'l'             # 'l' = 'r'-6       #
#>n{1S_8S_3v# 'o' = 'l'+3     # 'd' = 'l'-8       #
#v[_S8_S6_A<# ',' = 5*8+ 4    # '!' = 4*8+1       #
#>4n{1A_[..!# ' ' = 4*8       #                   <
$###########^######################################

 Multiply by 8 (Bidirectional)
n####v#####
>{8}m#m}8{<
n####^#####

 Push on left stack and print (Bidirectional)
_##v###
>{!#!{<
_##^###

 Add (Bidirectional)
A##v###
>}a#a}<
A##^###

 Subtract (Bidirectional)
S##v###
>}s#s}<
S##^###",
            )],
        }],
    };
