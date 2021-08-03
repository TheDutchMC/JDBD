use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jlong, jint};
use postgres::Client;
use crate::jni::util::set_error;

/**
 * - Class:      PostgreSqlDriver
 * - Method:     executeNative
 * - Signature:  `(JLjava/lang/String;)I`
 */
#[no_mangle]
pub extern "system" fn Java_dev_array21_jdbd_drivers_PostgreSqlDriver_executeNative(env: JNIEnv, obj: JObject<'_>, client_ptr: jlong, stmt: JString) -> jint {
    let stmt = match env.get_string(stmt) {
        Ok(s) => String::from(s),
        Err(e) => {
            set_error(env, obj, &format!("Failed to convert stmt from JString to String: {:?}", e));
            return -1;
        }
    };

    let mut client = unsafe { Box::from_raw(client_ptr as *mut Client)};

    match client.execute(&*stmt, &[]) {
        Ok(_) => {},
        Err(e) => {
            set_error(env, obj, &format!("Failed to execute stmt: {:?}", e));
            return -1;
        }
    }

    Box::into_raw(client);

    0
}