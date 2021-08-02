use jni::sys::{JNIEnv, jlong};
use jni::objects::JObject;
use mysql::Pool;

/**
 * - Class:      MysqlDriver
 * - Method:     unload
 * - Signature:  `(J)V`
 */
#[no_mangle]
pub extern "system" fn Java_MysqlDriver_unload__J(_env: JNIEnv, _obj: JObject, ptr: jlong) {
    let pool_ptr = ptr as *mut Pool;
    let pool = unsafe { Box::from_raw(pool_ptr) };
    drop(pool);
}