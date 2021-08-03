use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jlong, jint};
use mysql::{Pool, Params};
use crate::jni::util::set_error;
use mysql::prelude::Queryable;

/**
 * - Class:      MysqlDriver
 * - Method:     executeNative
 * - Signature:  `(JLjava/lang/String;)I`
 */
#[no_mangle]
pub extern "system" fn Java_dev_array21_jdbd_drivers_MysqlDriver_executeNative(env: JNIEnv, obj: JObject<'_>, pool_ptr: jlong, stmt: JString) -> jint {
    let pool_ptr = pool_ptr as *mut Pool;
    let mut conn = match unsafe { &*pool_ptr }.get_conn() {
        Ok(c) => c,
        Err(e) => {
            set_error(env, obj, &format!("Failed to create MySQL connection: {:?}", e));
            return -1;
        }
    };

    let stmt = match env.get_string(stmt) {
        Ok(s) => String::from(s),
        Err(e) => {
            set_error(env, obj, &format!("Failed to convert stmt from JString to String: {:?}", e));
            return -1;
        }
    };

    match conn.exec_drop(&stmt, Params::Empty) {
        Ok(_) => {},
        Err(e) => {
            set_error(env, obj, &format!("Failed to execute stmt: {:?}", e));
            return -1;
        }
    }

    0
}