//! Core traits shared across the SWed workspace.
//!
//! All traits use a generic value type `V` so that `swed_co` remains free of
//! any dependency on `swed_rt::HbValue`. Concrete crates (`swed_bf`, `swed_kn`)
//! implement these traits with `V = HbValue`.

mod interceptor;
mod module;
mod native_fn;
mod resolver;

pub use interceptor::ErrorInterceptor;
pub use module::ModuleComponent;
pub use native_fn::NativeFunction;
pub use resolver::FunctionResolver;
