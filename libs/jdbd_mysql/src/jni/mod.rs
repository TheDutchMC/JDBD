use jni::JNIEnv;
use jni::objects::{JObject, JValue};

pub mod execute;
pub mod initialize;
pub mod query;
pub mod unload;

pub fn set_error(env: JNIEnv, obj: JObject, e: &str) {
    eprintln!("{}", e);
    let jstr = match env.new_string(e.to_string()) {
        Ok(jstr) => jstr,
        Err(e) => {
            eprintln!("Failed to create new JString for error: {:?}", e);
            panic!("Failed to create new JString for error: {:?}", e)
        }
    };

    match env.set_field(obj, "errorBuffer", "Ljava/lang/String;", JValue::Object(jstr.into())) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Failed to set errorBuffer field: {:?}", e);
            panic!("Failed to set errorBuffer field: {:?}", e)
        }
    }
}