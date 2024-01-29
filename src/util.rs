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

mod res_helper;
pub use res_helper::ResHelper;