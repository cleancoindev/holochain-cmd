pub mod rust;
pub mod assemblyscript;

use error::DefaultResult;
use std::path::Path;

pub trait Scaffold {
    fn gen<P: AsRef<Path>>(&self, base_path: P) -> DefaultResult<()>;
}
