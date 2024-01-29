/**
 * util for scrcpy-clent
 * 
 * - error
 * - adb
 * - res_helper
 */

mod error;
pub use error::AppError;

mod adb;
pub use adb::Adb;
pub use adb::Device;

mod res_helper;
pub use res_helper::{ResHelper, ResourceName};