use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jlong, jint};
use postgres::Client;
use postgres::types::ToSql;
use crate::jni::common::SqlParameterArray;
use crate::jni::util::set_error;

/**
 * - Class:      PostgreSqlDriver
 * - Method:     executeNative
 * - Signature:  `(JLjava/lang/String;)I`
 */
#[no_mangle]
pub extern "system" fn Java_dev_array21_jdbd_drivers_PostgreSqlDriver_executeNative(env: JNIEnv, obj: JObject<'_>, client_ptr: jlong, stmt: JString, params: SqlParameterArray) -> jint {
    let stmt = match env.get_string(stmt) {
        Ok(s) => String::from(s),
        Err(e) => {
            set_error(env, obj, &format!("Failed to convert stmt from JString to String: {:?}", e));
            return -1;
        }
    };

    let mut client = unsafe { Box::from_raw(client_ptr as *mut Client)};

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

    let tosql_params=  sql_params_rust.iter().map(|x| x as &(dyn ToSql + Sync)).collect::<Vec<_>>();

    match client.execute(&*stmt, tosql_params.as_slice()) {
        Ok(_) => {},
        Err(e) => {
            set_error(env, obj, &format!("Failed to execute stmt: {:?}", e));
            return -1;
        }
    }

    Box::into_raw(client);

    0
}