use jni::sys::jlong;
use jni::objects::{JObject, JString};
use jni::JNIEnv;
use mysql::{Pool, OptsBuilder};
use super::set_error;

fn get_string_field(env: JNIEnv, obj: JObject, field_name: &'static str) -> Result<Option<String>, ()> {
    let field = match env.get_field(obj, &field_name, "Ljava/lang/String;") {
        Ok(f) => f,
        Err(e) => {
            let str_e = format!("Failed to retrieve field '{}': {:?}", field_name, e);
            set_error(env, obj, &str_e);
            return Err(());
        }
    };

    let as_jobject = match field.l() {
        Ok(o) => o,
        Err(e) => {
            let error = format!("Failed to convert field '{}' from JValue to JObject: {:?}", field_name, e);
            set_error(env, obj, &error);
            return Err(());
        }
    };
    let as_jstring = JString::from(as_jobject);
    if as_jstring.is_null() {
        return Ok(None);
    }

    match env.get_string(as_jstring) {
        Ok(str) => Ok(Some(str.into())),
        Err(e) => {
            let error = format!("Failed to convert field '{}' from JString to String: {:?}", field_name, e);
            set_error(env, obj, &error);
            return Err(());
        }
    }
}

/// Unwrap or return a nullptr
macro_rules! match_nullptr {
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
pub extern "system" fn Java_dev_array21_jdbd_drivers_MysqlDriver_initialize(env: JNIEnv, obj: JObject) -> jlong {
    let host = match_nullptr!(get_string_field(env, obj, "host"));
    let database = match_nullptr!(get_string_field(env, obj, "database"));

    let username = match_nullptr!(get_string_field(env, obj, "username"));
    let password = match_nullptr!(get_string_field(env, obj, "password"));

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