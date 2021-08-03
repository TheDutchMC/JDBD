use jni::JNIEnv;
use jni::objects::JObject;
use jni::sys::jlong;
use postgres::Client;

/**
 * - Class:      PostgreSqlDriver
 * - Method:     unloadNative
 * - Signature:  `(J)V`
 */
#[no_mangle]
pub extern "system" fn Java_dev_array21_jdbd_drivers_PostgreSqlDriver_unloadNative(_env: JNIEnv, _obj: JObject<'_>, ptr: jlong) {
    let client = unsafe { Box::from_raw(ptr as *mut Client) };
    drop(client);
}