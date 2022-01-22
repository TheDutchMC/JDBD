use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jlong, jint};
use mysql::{Pool, Params};
use crate::jni::util::set_error;
use mysql::prelude::Queryable;
use crate::jni::common::SqlParameterArray;

/**
 * - Class:      MysqlDriver
 * - Method:     executeNative
 * - Signature:  `(JLjava/lang/String;)I`
 */
#[no_mangle]
pub extern "system" fn Java_dev_array21_jdbd_drivers_MysqlDriver_executeNative(env: JNIEnv, obj: JObject<'_>, pool_ptr: jlong, stmt: JString, params: SqlParameterArray) -> jint {
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

    let sql_params_len = match env.get_array_length(params) {
        Ok(x) => x,
        Err(e) => {
            set_error(env, obj, &format!("Failed to get params array length: {:?}", e));
            return -1;
        }
    };

    let sql_params_java = match (0..sql_params_len).into_iter()
        .map(|x| env.get_object_array_element(params, x))
        .collect::<Result<Vec<_>, jni::errors::Error>>() {
        Ok(x) => x,
        Err(e) => {
            set_error(env, obj, &format!("Failed to fetch element of sql params array: {:?}", e));
            return -1;
        }
    };

    let sql_params_rust = match sql_params_java.into_iter()
        .map(|x| crate::jni::common::into_sql_parameter(&env, x))
        .collect::<Result<Vec<_>, jni::errors::Error>>() {
        Ok(x) => x,
        Err(e) => {
            set_error(env, obj, &format!("Failed to convert sql params element to Rust type: {:?}", e));
            return -1;
        }
    };

    let sql_params_mysql = sql_params_rust.into_iter()
        .map(|x| x.into())
        .collect::<Vec<mysql::Value>>();

    match conn.exec_drop(&stmt, Params::Positional(sql_params_mysql)) {
        Ok(_) => {},
        Err(e) => {
            set_error(env, obj, &format!("Failed to execute stmt: {:?}", e));
            return -1;
        }
    }

    0
}