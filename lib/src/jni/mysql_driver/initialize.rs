use jni::sys::jlong;
use jni::objects::JObject;
use jni::JNIEnv;
use mysql::{Pool, OptsBuilder};
use crate::jni::util::{set_error, get_string_field};
use crate::unwrap_zero;

/**
 * - Class:      MysqlDriver
 * - Method:     initializeNative
 * - Signature:  `()J`
 */
#[no_mangle]
pub extern "system" fn Java_dev_array21_jdbd_drivers_MysqlDriver_initializeNative(env: JNIEnv, obj: JObject<'_>) -> jlong {
    let host = unwrap_zero!(get_string_field(env, obj, "host"));
    let database = unwrap_zero!(get_string_field(env, obj, "database"));
    let username = unwrap_zero!(get_string_field(env, obj, "username"));
    let password = unwrap_zero!(get_string_field(env, obj, "password"));

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