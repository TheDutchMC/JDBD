use jni::JNIEnv;
use jni::objects::JObject;
use jni::sys::jlong;
use crate::jni::util::{set_error, get_string_field};
use crate::unwrap_zero;
use postgres::{Client, NoTls};

/**
 * - Class:      PostgreSqlDriver
 * - Method:     initializeNative
 * - Signature:  `()J`
 */
#[no_mangle]
pub extern "system" fn Java_dev_array21_jdbd_drivers_PostgreSqlDriver_initializeNative(env: JNIEnv, obj: JObject<'_>) -> jlong {
    let host = unwrap_zero!(get_string_field(env, obj, "host"));
    let database = unwrap_zero!(get_string_field(env, obj, "database"));
    let username = unwrap_zero!(get_string_field(env, obj, "username"));
    let password = unwrap_zero!(get_string_field(env, obj, "password"));

    let mut config = Client::configure();
    config.host(&host.unwrap());
    config.dbname(&database.unwrap());

    if let Some(username) = username {
        config.user(&username);
    }

    if let Some(password) = password {
        config.password(&password);
    }

    let client = match config.connect(NoTls) {
        Ok(c) => c,
        Err(e) => {
            set_error(env, obj, &format!("Failed to create PostgreSQL Client: {:?}", e));
            return 0;
        }
    };

    let rawptr = Box::into_raw(Box::new(client));
    rawptr as i64
}