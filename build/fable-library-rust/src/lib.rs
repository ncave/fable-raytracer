#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
#[path = "./Func.rs"]
pub(crate) mod import_346734d5;
pub use import_346734d5::*;
use crate::import_3bd9ae6a::*;
use crate::import_8d7d6be8::*;
use crate::import_c6216f2::*;
use crate::import_ec6ee4e9::*;
use crate::import_52af85ec::*;
use crate::import_f222008f::*;
#[path = "./Range.rs"]
pub(crate) mod import_3350cf54;
pub use import_3350cf54::*;
#[path = "./Result.rs"]
pub(crate) mod import_1a7b542;
pub use import_1a7b542::*;
#[path = "./Choice.rs"]
pub(crate) mod import_657750c0;
pub use import_657750c0::*;
#[path = "./Set.rs"]
pub(crate) mod import_52aff809;
pub use import_52aff809::*;
#[path = "./Map.rs"]
pub(crate) mod import_3b6ba757;
pub use import_3b6ba757::*;
use std::rc::Rc;
pub mod Fable_Library_Rust {
    use super::*;
    pub fn imports() -> Rc<MutCell<Vec<()>>> {
        Native::arrayFrom(&[(), (), (), (), (), (), (), (), (), (), (), ()])
    }
}
