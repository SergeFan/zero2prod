pub use health_check::*;
pub use newsletters::*;
use std::error::Error;
use std::fmt::Formatter;
pub use subscriptions::*;
pub use subscriptions_confirm::*;

mod health_check;
mod newsletters;
mod subscriptions;
mod subscriptions_confirm;

pub fn error_chain_fmt(e: &impl Error, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;

    let mut current = e.source();

    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;

        current = cause.source();
    }

    Ok(())
}
