use jni::sys::jlong;
use jni::objects::JObject;
use jni::JNIEnv;
use mysql::{Pool, OptsBuilder};
use crate::jni::util::{set_error, get_string_field};

/// Unwrap or return 0
macro_rules! match_zero {
    ($i:expr) => {
        match $i {
            Ok(i) => i,
            Err(_) => return 0
        }
    }
}

/**
 * - Class:      MysqlDriver
 * - Method:     initialize
 * - Signature:  `()J`
 */
#[no_mangle]
pub extern "system" fn Java_dev_array21_jdbd_drivers_MysqlDriver_initializeNative(env: JNIEnv, obj: JObject) -> jlong {
    let host = match_zero!(get_string_field(env, obj, "host"));
    let database = match_zero!(get_string_field(env, obj, "database"));

    let username = match_zero!(get_string_field(env, obj, "username"));
    let password = match_zero!(get_string_field(env, obj, "password"));

    let cfg = OptsBuilder::new()
        .ip_or_hostname(host)
        .user(username)
        .pass(password)
        .db_name(database);

    let pool = match Pool::new(cfg) {
        Ok(p) => p,
        Err(e) => {
            let error = format!("Failed to create MySQL Connection Pool: {:?}", e);
            set_error(env, obj, &error);
            return 0;
        }
    };

    let rawptr = Box::into_raw(Box::new(pool));
    rawptr as i64
}