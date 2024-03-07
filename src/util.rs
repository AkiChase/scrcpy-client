/**
 * util for scrcpy-clent
 *
 * - adb
 * - res_helper
 * - socket
 */

mod adb;
pub use adb::{Adb, Device};

mod res_helper;
pub use res_helper::{ResHelper, ResourceName};
