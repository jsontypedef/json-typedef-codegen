pub mod go;
pub mod rust;
pub mod typescript;

use clap::{App, ArgMatches};
use failure::Error;
use jtd::Schema;

pub trait Target
where
    Self: Sized,
{
    fn args<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b>;
    fn from_args(matches: &ArgMatches) -> Result<Option<Self>, Error>;
    fn codegen(&self, schema: &Schema) -> Result<(), Error>;
}
