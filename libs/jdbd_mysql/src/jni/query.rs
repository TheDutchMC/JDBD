use jni::sys::{jobjectArray, jlong};
use jni::objects::{JObject, JString, JClass, JValue};
use mysql::{Pool, Params, Row};
use super::set_error;
use jni::JNIEnv;
use std::ptr::null_mut;
use mysql::prelude::Queryable;
use mysql::consts::ColumnType;

macro_rules! to_jstring {
    ($env:expr, $obj:expr, $str:expr) => {
        match $env.new_string($str) {
            Ok(s) => s,
            Err(e) => {
                $crate::jni::set_error($env, $obj, &format!("Failed to convert String to JString: {:?}", e));
                return ::std::ptr::null_mut()
            }
        }
    }
}

macro_rules! find_class {
    ($env:expr, $obj:expr, $class:expr) => {
        match $env.find_class($class) {
            Ok(c) => c,
            Err(e) => {
                $crate::jni::set_error($env, $obj, &format!("Unable to find class '{}': {:?}", $class, e));
                return ::std::ptr::null_mut();
            }
        }
    }
}

/**
 * - Class:      MysqlDriver
 * - Method:     query
 * - Signature:  `(JLjava/lang/String;)[Ldev/array21/jdbd/datatypes/SqlRow;`
 */
#[no_mangle]
pub extern "system" fn Java_dev_array21_jdbd_drivers_MysqlDriver_queryNative(env: JNIEnv, obj: JObject, pool_ptr: jlong, stmt: JString) -> jobjectArray {
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

    let result = match conn.exec::<Row, &str, Params>(&stmt, Params::Empty) {
        Ok(r) => r,
        Err(e) => {
            set_error(env, obj, &format!("Failed to execute stmt: {:?}", e));
            return null_mut();
        }
    };

    let sqlrow_class = find_class!(env, obj, "dev/array21/jdbd/datatypes/SqlRow");
    let sqlrow_arr = match env.new_object_array(result.len() as i32, sqlrow_class, JObject::null()) {
        Ok(r) => r,
        Err(e) => {
            set_error(env, obj, &format!("Failed to create dev.array21.jdbd.datatypes.SqlRow[]: {:?}", e));
            return null_mut();
        }
    };

    let string_class = find_class!(env, obj, "java/lang/String");
    let long_class = find_class!(env, obj, "java/lang/Long");
    let double_class = find_class!(env, obj, "java/lang/Double");
    let byte_class = find_class!(env, obj, "java/lang/Byte");
    let byte_array_class = {
        let obj_arr = match env.new_object_array(0, byte_class, JObject::null()) {
            Ok(oa) => oa,
            Err(e) => {
                set_error(env, obj, &format!("Failed to create new java.lang.Byte[]: {:?}", e));
                return null_mut();
            }
        };
        match env.get_object_class(obj_arr) {
            Ok(c) => c,
            Err(e) => {
                set_error(env, obj, &format!("Failed to get class from java.lang.Byte[]: {:?}", e));
                return null_mut();
            }
        }
    };

    let mut row_index = 0;
    for row in result {
        let mut classes: Vec<JClass> = Vec::new();
        let mut objects: Vec<JObject> = Vec::new();
        let mut names: Vec<JString> = Vec::new();

        for col in row.columns().iter() {
            let name = col.name_str();
            match col.column_type() {
                ColumnType::MYSQL_TYPE_STRING | ColumnType::MYSQL_TYPE_VAR_STRING | ColumnType::MYSQL_TYPE_VARCHAR => {
                    let v: Option<String> = row.get(&*name).unwrap();
                    classes.push(string_class);
                    names.push(to_jstring!(env, obj, name));
                    match v {
                        Some(v) => objects.push(to_jstring!(env, obj, v).into()),
                        None => objects.push(JObject::null())
                    }
                },
                ColumnType::MYSQL_TYPE_INT24 | ColumnType::MYSQL_TYPE_LONG | ColumnType::MYSQL_TYPE_LONGLONG => {
                    let v: Option<i64> = row.get(&*name).unwrap();
                    classes.push(long_class);
                    names.push(to_jstring!(env, obj, name));
                    match v {
                        Some(v) => {
                            match env.new_object(long_class, "(J)V", &[JValue::Long(v)]) {
                                Ok(long) => objects.push(long),
                                Err(e) => {
                                    set_error(env, obj, &format!("Failed to convert i64 to java.lang.Long: {:?}", e));
                                    return null_mut();
                                }
                            }
                        },
                        None => objects.push(JObject::null())
                    }
                },
                ColumnType::MYSQL_TYPE_DOUBLE | ColumnType::MYSQL_TYPE_FLOAT => {
                    let v: Option<f64> = row.get(&*name).unwrap();
                    classes.push(double_class);
                    names.push(to_jstring!(env, obj, name));
                    match v {
                        Some(v) => {
                            match env.new_object(double_class, "(D)V", &[JValue::Double(v)]) {
                                Ok(double) => objects.push(double),
                                Err(e) => {
                                    set_error(env, obj, &format!("Failed to convert f64 to java.lang.Double: {:?}", e));
                                    return null_mut();
                                }
                            }
                        },
                        None => objects.push(JObject::null())
                    }
                },
                ColumnType::MYSQL_TYPE_BLOB | ColumnType::MYSQL_TYPE_LONG_BLOB | ColumnType::MYSQL_TYPE_MEDIUM_BLOB | ColumnType::MYSQL_TYPE_TINY_BLOB => {
                    let v: Option<Vec<u8>> = row.get(&*name).unwrap();
                    classes.push(byte_array_class);
                    names.push(to_jstring!(env, obj, name));
                    match v {
                        Some(v) => {
                            let mut jbytes: Vec<JObject> = Vec::new();
                            for byte in v {
                                match env.new_object(byte_class, "(B)V", &[JValue::Byte(byte as i8)]) {
                                    Ok(jbyte) => jbytes.push(jbyte),
                                    Err(e) => {
                                        set_error(env, obj, &format!("Failed to convert u8 to java.lang.Byte: {:?}", e));
                                        return null_mut();
                                    }
                                }
                            }

                            let array = match env.new_object_array(jbytes.len() as i32, byte_class, JObject::null()) {
                                Ok(a) => a,
                                Err(e) => {
                                    set_error(env, obj, &format!("Failed to create new java.lang.Byte[]: {:?}", e));
                                    return null_mut();
                                }
                            };

                            let mut index = 0;
                            for jbyte in jbytes.drain(..) {
                                index += 1;
                                match env.set_object_array_element(array, index, jbyte) {
                                    Ok(_) => {},
                                    Err(e) => {
                                        set_error(env, obj, &format!("Failed to set object in java.lang.Byte[]: {:?}", e));
                                        return null_mut();
                                    }
                                }
                            }

                            objects.push(JObject::from(array));
                        },
                        None => objects.push(JObject::null())
                    }
                }
                _ => unimplemented!()
            }


        }

        let names_arr = match env.new_object_array(names.len() as i32, string_class, JObject::null()) {
            Ok(a) => a,
            Err(e) => {
                set_error(env, obj, &format!("Failed to create new java.lang.String[]: {:?}", e));
                return null_mut();
            }
        };

        let objs_arr = match env.new_object_array(objects.len() as i32, find_class!(env, obj, "java/lang/Object"), JObject::null()) {
            Ok(a) => a,
            Err(e) => {
                set_error(env, obj, &format!("Failed to create new java.lang.Object[]: {:?}", e));
                return null_mut();
            }
        };

        let classes_arr = match env.new_object_array(classes.len() as i32, find_class!(env, obj, "java/lang/Class"), JObject::null()) {
            Ok(a) => a,
            Err(e) => {
                set_error(env, obj, &format!("Failed to create new java.lang.Class[]: {:?}", e));
                return null_mut();
            }
        };

        let mut index = 0;
        for ((class, object), name) in classes.drain(..).zip(objects.drain(..)).zip(names.drain(..)) {
            match env.set_object_array_element(names_arr, index, name) {
                Ok(_) => {},
                Err(e) => {
                    set_error(env, obj, &format!("Failed to set object in java.lang.String[]: {:?}", e));
                    return null_mut();
                }
            }

            match env.set_object_array_element(objs_arr, index, object) {
                Ok(_) => {},
                Err(e) => {
                    set_error(env, object, &format!("Failed to set object in java.lang.Object[]: {:?}", e));
                    return null_mut();
                }
            }

            match env.set_object_array_element(classes_arr, index, class) {
                Ok(_) => {},
                Err(e) => {
                    set_error(env, obj, &format!("Failed to set object in java.lang.Class[]: {:?}", e));
                    return null_mut();
                }
            }

            index +=1;
        }

        let sql_row = match env.new_object(sqlrow_class, "([Ljava/lang/String;[Ljava/lang/Object;[Ljava/lang/Class;)V", &[JValue::Object(names_arr.into()), JValue::Object(objs_arr.into()), JValue::Object(classes_arr.into())]) {
            Ok(r) => r,
            Err(e) => {
                set_error(env, obj, &format!("Failed to create new SqlRow object: {:?}", e));
                return null_mut();
            }
        };

        match env.set_object_array_element(sqlrow_arr, row_index, sql_row) {
            Ok(_) => {},
            Err(e) => {
                set_error(env, obj, &format!("Failed to set object in dev.array21.jdbd.datatypes.SqlRow[]: {:?}", e));
                return null_mut();
            }
        }
        row_index += 1;
    }

    sqlrow_arr
}
