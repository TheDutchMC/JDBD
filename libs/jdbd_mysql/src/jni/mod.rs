use jni::JNIEnv;
use jni::objects::{JObject, JValue};

pub mod execute;
pub mod initialize;
pub mod query;
pub mod unload;

pub fn set_error(env: JNIEnv, obj: JObject, e: &str) {
    let jstr = match env.new_string(e) {
        Ok(jstr) => jstr,
        Err(e) => panic!("{:?}", e)
    };

    match env.set_field(obj, "errorBuffer", "Ljava/lang/String;", JValue::Object(jstr.into())) {
        Ok(_) => {},
        Err(e) => panic!("{:?}", e)
    }
}