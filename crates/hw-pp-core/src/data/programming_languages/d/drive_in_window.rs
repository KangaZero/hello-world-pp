//! Hello-world snippet for `ProgrammingLanguageName::DriveInWindow`.
//!
//! Imported from leachim6/hello-world (`d/Drive-In Window.diw` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/Drive-In%20Window.diw>
//! - <https://esolangs.org/wiki/Drive-In_Window> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: DriveInWindow => "Drive-In Window",
    versions: [DriveInWindow => "Drive-In Window"],
    extensions: [Diw => "diw"],
}

const DRIVE_IN_WINDOW: &str = r"Hi, welcome to Hello_World_Buffet. Here is a menu.

Rice Krispie Treat: $10
Ice Cream: $30
Cookies: $70

Here are your sides.

Whipped Cream: $1
Chocolate: $2
Caramel: $3

May I take your order?

Person 1 would like the Cookies with Chocolate.
Person 1 will pay for his order!
Person 2 would like the Ice Cream, hold the Whipped Cream.
Person 2 would also like what Person 1 has.
Person 2 will pay for his order!
Person 2 needs 7 dollars more for his order!
Person 2 will pay for his order!
Person 2 will pay for his order!
Person 2 needs 3 dollars more for his order!
Person 2 will pay for his order!
Person 3 would like the Ice Cream with Chocolate.
Person 3 will pay for his order!
Person 1 would also like the Rice Krispie Treat with Caramel.
Person 1 needs 2 more dollars for his order!
Person 1 will pay for his order!
Person 2 will pay for his order!
Person 2 needs 3 dollars more for his order!
Person 2 will pay for his order!
Person 2 needs 6 dollars less for his order!
Person 2 will pay for his order!
Person 2 needs 8 dollars less for his order!
Person 2 will pay for his order!
Person 3 needs 1 dollar more for his order!
Person 3 will pay for his order!
Just wait while we decide...

OK, that will be $1.14. Thanks for coming!";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::DriveInWindow,
        extensions: &[FileExtension::Diw],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::DriveInWindow,
            syntax: DRIVE_IN_WINDOW,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DRIVE_IN_WINDOW,
                    [(
                        TokenKind::Plain,
                        r"Hi, welcome to Hello_World_Buffet. Here is a menu.

Rice Krispie Treat: $10
Ice Cream: $30
Cookies: $70

Here are your sides.

Whipped Cream: $1
Chocolate: $2
Caramel: $3

May I take your order?

Person 1 would like the Cookies with Chocolate.
Person 1 will pay for his order!
Person 2 would like the Ice Cream, hold the Whipped Cream.
Person 2 would also like what Person 1 has.
Person 2 will pay for his order!
Person 2 needs 7 dollars more for his order!
Person 2 will pay for his order!
Person 2 will pay for his order!
Person 2 needs 3 dollars more for his order!
Person 2 will pay for his order!
Person 3 would like the Ice Cream with Chocolate.
Person 3 will pay for his order!
Person 1 would also like the Rice Krispie Treat with Caramel.
Person 1 needs 2 more dollars for his order!
Person 1 will pay for his order!
Person 2 will pay for his order!
Person 2 needs 3 dollars more for his order!
Person 2 will pay for his order!
Person 2 needs 6 dollars less for his order!
Person 2 will pay for his order!
Person 2 needs 8 dollars less for his order!
Person 2 will pay for his order!
Person 3 needs 1 dollar more for his order!
Person 3 will pay for his order!
Just wait while we decide...

OK, that will be $1.14. Thanks for coming!",
                    )],
                )
            },
        }],
    };
