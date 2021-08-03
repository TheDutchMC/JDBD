use jni::JNIEnv;
use jni::objects::{JObject, JString, JClass};
use jni::sys::{jlong, jobjectArray};
use postgres::Client;
use crate::jni::util::{set_error, Java};
use std::ptr::null_mut;
use crate::unwrap_nullptr;

/**
 * - Class:      PostgreSqlDriver
 * - Method:     queryNative
 * - Signature:  `(JLjava/lang/String;)[Ldev/array21/jdbd/datatypes/SqlRow;`
 */
#[no_mangle]
pub extern "system" fn Java_dev_array21_jdbd_drivers_PostgreSqlDriver_queryNative(env: JNIEnv, obj: JObject<'_>, client_ptr: jlong, stmt: JString) -> jobjectArray {
    let stmt = match env.get_string(stmt) {
        Ok(s) => String::from(s),
        Err(e) => {
            set_error(env, obj, &format!("Failed to convert stmt from JString to String: {:?}", e));
            return null_mut()
        }
    };

    let mut client = unsafe { Box::from_raw(client_ptr as *mut Client) };
    let result = match client.query(&*stmt, &[]) {
        Ok(r) => r,
        Err(e) => {
            set_error(env, obj, &format!("Failed to execute stmt: {:?}", e));
            return null_mut();
        }
    };

    Box::into_raw(client);

    // Vec of dev.array21.jdbd.datatypes.SqlRow
    let mut sqlrows = Vec::new();

    for row in result {
        // Vec of java.lang.Class
        let mut classes: Vec<JClass> = Vec::new();
        // Vec of java.lang.Object
        let mut objects: Vec<JObject> = Vec::new();
        let mut names: Vec<String> = Vec::new();

        for col in row.columns().iter() {
            let name = col.name();

            match col.type_().name() {
                "text" | "varchar" | "bytea" => {
                    let v: Option<String> = row.get(&name);
                    classes.push(unwrap_nullptr!(env, obj, Java::String(env)));
                    names.push(name.to_string());
                    match v {
                        Some(v) => {
                            let string = unwrap_nullptr!(env, obj, Java::new_String(env, v));
                            objects.push(string.into());
                        }
                        None => objects.push(JObject::null())
                    }
                },
                "int8" | "int2" | "int4" | "numeric" => {
                    let v: Option<i64> = row.get(&name);
                    classes.push(unwrap_nullptr!(env, obj, Java::Long(env)));
                    names.push(name.to_string());
                    match v {
                        Some(v) => {
                            let long = unwrap_nullptr!(env, obj, Java::new_Long(env, v));
                            objects.push(long);
                        },
                        None => objects.push(JObject::null())
                    }
                },
                "float4" | "float8" => {
                    let v: Option<f64> = row.get(&*name);
                    classes.push(unwrap_nullptr!(env, obj, Java::Double(env)));
                    names.push(name.to_string());
                    match v {
                        Some(v) => {
                            let double = unwrap_nullptr!(env, obj, Java::new_Double(env, v));
                            objects.push(double);
                        },
                        None => objects.push(JObject::null())
                    }
                },
                "_bytea" => {
                    let v: Option<Vec<u8>> = row.get(&*name);
                    classes.push(unwrap_nullptr!(env, obj, Java::Byte_array(env)));
                    names.push(name.to_string());
                    match v {
                        Some(v) => {
                            let byte_array = unwrap_nullptr!(env, obj, Java::new_Byte_array_u8(env, v));
                            objects.push(byte_array);
                        },
                        None => objects.push(JObject::null())
                    }
                },
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