#[cfg(not(feature = "demo"))]
mod http;
#[cfg(not(feature = "demo"))]
pub use http::*;

#[cfg(feature = "demo")]
mod mock;
#[cfg(feature = "demo")]
pub use mock::*;
