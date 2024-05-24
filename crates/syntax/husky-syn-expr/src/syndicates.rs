pub mod affect;
pub mod be_variable;
pub mod case_variable;
pub mod closure_parameter;
pub mod let_variable;
pub mod parenate_parameter;
pub mod props_field;
pub mod return_ty;
pub mod self_ty;
pub mod self_value_parameter;
pub mod template_parameter;
pub mod trai;
pub mod trais;
pub mod tuple_field;

pub use self::be_variable::*;
pub use self::case_variable::*;
pub use self::let_variable::*;
pub use self::parenate_parameter::*;
pub use self::props_field::*;
pub use self::return_ty::*;
pub use self::self_ty::*;
pub use self::self_value_parameter::*;
pub use self::template_parameter::*;
pub use self::trai::*;
pub use self::tuple_field::*;

use crate::*;
use parsec::*;
