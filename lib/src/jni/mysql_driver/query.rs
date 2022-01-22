use jni::sys::{jobjectArray, jlong};
use jni::objects::{JObject, JString, JClass};
use mysql::{Pool, Params, Row};
use crate::jni::util::{set_error, Java};
use jni::JNIEnv;
use std::ptr::null_mut;
use mysql::prelude::Queryable;
use mysql::consts::ColumnType;
use crate::jni::common::SqlParameterArray;
use crate::unwrap_nullptr;

/**
 * - Class:      MysqlDriver
 * - Method:     queryNative
 * - Signature:  `(JLjava/lang/String;)[Ldev/array21/jdbd/datatypes/SqlRow;`
 */
#[no_mangle]
pub extern "system" fn Java_dev_array21_jdbd_drivers_MysqlDriver_queryNative(env: JNIEnv, obj: JObject<'_>, pool_ptr: jlong, stmt: JString, params: SqlParameterArray) -> jobjectArray {
    let pool_ptr = pool_ptr as *mut Pool;
    let mut conn = match unsafe { &*pool_ptr }.get_conn() {
        Ok(c) => c,
        Err(e) => {
            set_error(env, obj, &format!("Failed to create MySQL connection: {:?}", e));
            return null_mut();
        }
    };

    let stmt = match env.get_string(stmt) {
        Ok(s) => String::from(s),
        Err(e) => {
            set_error(env, obj, &format!("Failed to convert stmt from JString to String: {:?}", e));
            return null_mut()
        }
    };

    let sql_params_len = match env.get_array_length(params) {
        Ok(x) => x,
        Err(e) => {
            set_error(env, obj, &format!("Failed to get params array length: {:?}", e));
            return null_mut();
        }
    };

    let sql_params_java = match (0..sql_params_len).into_iter()
        .map(|x| env.get_object_array_element(params, x))
        .collect::<Result<Vec<_>, jni::errors::Error>>() {
        Ok(x) => x,
        Err(e) => {
            set_error(env, obj, &format!("Failed to fetch element of sql params array: {:?}", e));
            return null_mut();
        }
    };

    let sql_params_rust = match sql_params_java.into_iter()
        .map(|x| crate::jni::common::into_sql_parameter(&env, x))
        .collect::<Result<Vec<_>, jni::errors::Error>>() {
        Ok(x) => x,
        Err(e) => {
            set_error(env, obj, &format!("Failed to convert sql params element to Rust type: {:?}", e));
            return null_mut();
        }
    };
    let sql_params_mysql = sql_params_rust.into_iter()
        .map(|x| x.into())
        .collect::<Vec<mysql::Value>>();

    let result = match conn.exec::<Row, &str, Params>(&stmt, Params::Positional(sql_params_mysql)) {
        Ok(r) => r,
        Err(e) => {
            set_error(env, obj, &format!("Failed to execute stmt: {:?}", e));
            return null_mut();
        }
    };

    // Vec of dev.array21.jdbd.datatypes.SqlRow
    let mut sqlrows = Vec::new();

    for row in result {
        // Vec of java.lang.Class
        let mut classes: Vec<JClass> = Vec::new();
        // Vec of java.lang.Object
        let mut objects: Vec<JObject> = Vec::new();
        let mut names: Vec<String> = Vec::new();

        for col in row.columns().iter() {
            let name = col.name_str().to_string();
            match col.column_type() {
                ColumnType::MYSQL_TYPE_STRING | ColumnType::MYSQL_TYPE_VAR_STRING | ColumnType::MYSQL_TYPE_VARCHAR => {
                    let v: Option<String> = row.get(&*name).unwrap();
                    classes.push(unwrap_nullptr!(env, obj, Java::String(env)));
                    names.push(name);
                    match v {
                        Some(v) => {
                            let string = unwrap_nullptr!(env, obj, Java::new_String(env, v));
                            objects.push(string.into());
                        }
                        None => objects.push(JObject::null())
                    }
                },
                ColumnType::MYSQL_TYPE_INT24 | ColumnType::MYSQL_TYPE_LONG | ColumnType::MYSQL_TYPE_LONGLONG => {
                    let v: Option<i64> = row.get(&*name).unwrap();
                    classes.push(unwrap_nullptr!(env, obj, Java::Long(env)));
                    names.push(name);
                    match v {
                        Some(v) => {
                            let long = unwrap_nullptr!(env, obj, Java::new_Long(env, v));
                            objects.push(long);
                        },
                        None => objects.push(JObject::null())
                    }
                },
                ColumnType::MYSQL_TYPE_DOUBLE | ColumnType::MYSQL_TYPE_FLOAT => {
                    let v: Option<f64> = row.get(&*name).unwrap();
                    classes.push(unwrap_nullptr!(env, obj, Java::Double(env)));
                    names.push(name);
                    match v {
                        Some(v) => {
                            let double = unwrap_nullptr!(env, obj, Java::new_Double(env, v));
                            objects.push(double);
                        },
                        None => objects.push(JObject::null())
                    }
                },
                ColumnType::MYSQL_TYPE_BLOB | ColumnType::MYSQL_TYPE_LONG_BLOB | ColumnType::MYSQL_TYPE_MEDIUM_BLOB | ColumnType::MYSQL_TYPE_TINY_BLOB => {
                    let v: Option<Vec<u8>> = row.get(&*name).unwrap();
                    classes.push(unwrap_nullptr!(env, obj, Java::Byte_array(env)));
                    names.push(name);
                    match v {
                        Some(v) => {
                            let byte_array = unwrap_nullptr!(env, obj, Java::new_Byte_array_u8(env, v));
                            objects.push(byte_array);
                        },
                        None => objects.push(JObject::null())
                    }
                }
                _ => unimplemented!()
            }
        }

        // Type = dev.array21.jdbd.datatypes.SqlRow
        let sqlrow_java = unwrap_nullptr!(env, obj, Java::new_SqlRow(env, names, objects, classes));
        sqlrows.push(sqlrow_java);
    }

    // Convert the Vec of dev.array21.jdbd.datatypes.SqlRow to dev.array21.jdbd.datatypes.SqlRow[]
    let sqlrow_java_array = unwrap_nullptr!(env, obj, Java::new_SqlRow_array(env, sqlrows));
    sqlrow_java_array.into_inner()
}
