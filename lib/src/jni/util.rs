use jni::JNIEnv;
use jni::objects::{JObject, JValue, JClass, JString};

/// Set the errorBuffer field in the provided Object
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

/// Find a class, or if it can not be found set an error and return a nullptr
#[macro_export]
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

#[macro_export]
macro_rules! unwrap_nullptr {
    ($env:expr, $obj:expr, $expr:expr) => {
        match $expr {
            Ok(e) => e,
            Err(e) => {
                $crate::jni::util::set_error($env, $obj, &e);
                return ::std::ptr::null_mut();
            }
        }
    }
}

/// Unwrap or return 0
#[macro_export]
macro_rules! unwrap_zero {
    ($i:expr) => {
        match $i {
            Ok(i) => i,
            Err(_) => return 0
        }
    }
}

/// Get the value of a String Field from the provided Object
pub fn get_string_field(env: JNIEnv, obj: JObject, field_name: &'static str) -> Result<Option<String>, ()> {
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

pub struct Java();

impl Java {
    #![allow(non_snake_case)]   // We permit this so that we can adhere to Java's naming scheme for classes and clearly differentiate between e.g byte (primitive) and Byte (Object)
    #![allow(unused)]

    pub fn Byte_array(env: JNIEnv) -> Result<JClass, String> {
        let Byte_array = match env.new_object_array(0, Self::Byte(env)?, JObject::null()) {
            Ok(a) => a,
            Err(e) => return Err(format!("Failed to create new java.lang.Byte[]: {:?}", e))
        };

        let class = match env.get_object_class(Byte_array) {
            Ok(c) => c,
            Err(e) => return Err(format!("Failed to get class from java.lang.Byte[]: {:?}", e))
        };

        Ok(class)
    }

    pub fn String(env: JNIEnv) -> Result<JClass, String> {
        match env.find_class("java/lang/String") {
            Ok(c) => Ok(c),
            Err(e) => Err(format!("Failed to get class java.lang.String: {:?}", e))
        }
    }

    pub fn Double(env: JNIEnv) -> Result<JClass, String> {
        match env.find_class("java/lang/Double") {
            Ok(c) => Ok(c),
            Err(e) => Err(format!("Failed to get class java.lang.Double: {:?}", e))
        }
    }

    pub fn Boolean(env: JNIEnv) -> Result<JClass, String> {
        match env.find_class("java/lang/Boolean") {
            Ok(c) => Ok(c),
            Err(e) => Err(format!("Failed to get class java.lang.Boolean: {:?}", e))
        }
    }

    pub fn Object(env: JNIEnv) -> Result<JClass, String> {
        match env.find_class("java/lang/Object") {
            Ok(c) => Ok(c),
            Err(e) => Err(format!("Failed to get class java.lang.Object: {:?}", e))
        }
    }

    pub fn Byte(env: JNIEnv) -> Result<JClass, String> {
        match env.find_class("java/lang/Byte") {
            Ok(c) => Ok(c),
            Err(e) => Err(format!("Failed to get class java.lang.Byte: {:?}", e))
        }
    }

    pub fn Long(env: JNIEnv) -> Result<JClass, String> {
        match env.find_class("java/lang/Long") {
            Ok(c) => Ok(c),
            Err(e) => Err(format!("Failed to get class java.lang.Long: {:?}", e))
        }
    }

    pub fn Class(env: JNIEnv) -> Result<JClass, String> {
        match env.find_class("java/lang/Class") {
            Ok(c) => Ok(c),
            Err(e) => Err(format!("Failed to get class java.lang.Class: {:?}", e))
        }
    }

    pub fn SqlRow(env: JNIEnv) -> Result<JClass, String> {
        match env.find_class("dev/array21/jdbd/datatypes/SqlRow") {
            Ok(c) => Ok(c),
            Err(e) => Err(format!("Failed to get class dev.array21.jdbd.datatypes.SqlRow: {:?}", e))
        }
    }

    pub fn new_String(env: JNIEnv, string: String) -> Result<JString, String> {
        match env.new_string(string) {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("Failed to create new java.lang.String: {:?}", e))
        }
    }

    pub fn new_Long(env: JNIEnv, long: i64) -> Result<JObject<'_>, String> {
        match env.new_object(Self::Long(env)?, "(J)V", &[JValue::Long(long)]) {
            Ok(long) => Ok(long),
            Err(e) => Err(format!("Failed to convert i64 to java.lang.Long: {:?}", e))
        }
    }

    pub fn new_Byte(env: JNIEnv, byte: u8) -> Result<JObject<'_>, String> {
        match env.new_object(Self::Byte(env)?, "(B)V", &[JValue::Byte(byte as i8)]) {
            Ok(byte) => Ok(byte),
            Err(e) => Err(format!("Failed to convert u8 to java.lang.Byte: {:?}", e))
        }
    }

    pub fn new_Double(env: JNIEnv, double: f64) -> Result<JObject<'_>, String> {
        match env.new_object(Self::Double(env)?, "(D)V", &[JValue::Double(double)]) {
            Ok(double) => Ok(double),
            Err(e) => Err(format!("Failed to convert f64 to java.lang.Double: {:?}", e))
        }
    }

    pub fn new_Byte_array_obj<'a>(env: JNIEnv, bytes: Vec<JObject<'a>>) -> Result<JObject<'a>, String> {
        let array = match env.new_object_array(bytes.len() as i32, Self::Byte(env)?, JObject::null()) {
            Ok(a) => a,
            Err(e) => return Err(format!("Failed to create new java.lang.Byte[]: {:?}", e))
        };

        let mut index = 0;
        for byte in bytes.into_iter() {
            match env.set_object_array_element(array, index, byte) {
                Ok(_) => {},
                Err(e) => return Err(format!("Failed to set object in java.lang.Byte[]: {:?}", e))
            }

            index += 1;
        }

        Ok(JObject::from(array))
    }

    pub fn new_Byte_array_u8(env: JNIEnv, bytes: Vec<u8>) -> Result<JObject<'_>, String> {
        let mut jbytes = Vec::new();
        for byte in bytes {
            jbytes.push(Self::new_Byte(env, byte)?)
        }

        Self::new_Byte_array_obj(env, jbytes)
    }

    pub fn new_String_array_obj<'a>(env: JNIEnv, strings: Vec<JString>) -> Result<JObject<'a>, String> {
        let array = match env.new_object_array(strings.len() as i32, Self::String(env)?, JObject::null()) {
            Ok(a) => a,
            Err(e) => return Err(format!("Failed to create new java.lang.String[]: {:?}", e))
        };

        let mut index = 0;
        for string in strings.into_iter() {
            match env.set_object_array_element(array, index, JObject::from(string)) {
                Ok(_) => {},
                Err(e) => return Err(format!("Failed to set object in java.lang.String[]: {:?}", e))
            }

            index += 1;
        }

        Ok(JObject::from(array))
    }

    pub fn new_String_array_string<T: AsRef<str>>(env: JNIEnv, strings: Vec<T>) -> Result<JObject<'_>, String> {
        let mut jstrings = Vec::new();
        for string in strings {
            let string = string.as_ref().to_string();
            jstrings.push(Java::new_String(env, string)?)
        }
        Self::new_String_array_obj(env, jstrings)
    }

    pub fn new_Object_array<'a>(env: JNIEnv, objects: Vec<JObject<'a>>) -> Result<JObject<'a>, String> {
        let array = match env.new_object_array(objects.len() as i32, Self::Object(env)?, JObject::null()) {
            Ok(a) => a,
            Err(e) => return Err(format!("Failed to create new java.lang.Object[]: {:?}", e))
        };

        let mut index = 0;
        for object in objects.into_iter() {
            match env.set_object_array_element(array, index, object) {
                Ok(_) => {},
                Err(e) => return Err(format!("Failed to set object in java.lang.Object[]: {:?}", e))
            }

            index += 1;
        }

        Ok(JObject::from(array))
    }

    pub fn new_Class_array<'a>(env: JNIEnv, classes: Vec<JClass<'a>>) -> Result<JObject<'a>, String> {
        let array = match env.new_object_array(classes.len() as i32, Self::Class(env)?, JObject::null()) {
            Ok(a) => a,
            Err(e) => return Err(format!("Failed to create new java.lang.Class[]: {:?}", e))
        };

        let mut index = 0;
        for class in classes.into_iter() {
            match env.set_object_array_element(array, index, JObject::from(class)) {
                Ok(_) => {},
                Err(e) => return Err(format!("Failed to set object in java.lang.Class[]: {:?}", e))
            }

            index += 1;
        }

        Ok(JObject::from(array))
    }

    pub fn new_SqlRow<'a>(env: JNIEnv<'a>, names: Vec<String>, objects: Vec<JObject<'a>>, classes: Vec<JClass<'a>>) -> Result<JObject<'a>, String> {
        let names = JValue::Object(Java::new_String_array_string(env, names)?);
        let objects = JValue::Object(Java::new_Object_array(env, objects)?);
        let classes = JValue::Object(Java::new_Class_array(env, classes)?);

        match env.new_object(Self::SqlRow(env)?, "([Ljava/lang/String;[Ljava/lang/Object;[Ljava/lang/Class;)V", &[names, objects, classes]) {
            Ok(r) => Ok(r),
            Err(e) => Err(format!("Failed to create new dev.array21.jdbd.datatypes.SqlRow: {:?}", e))
        }
    }

    pub fn new_SqlRow_array<'a>(env: JNIEnv, rows: Vec<JObject<'a>>) -> Result<JObject<'a>, String> {
        let array = match env.new_object_array(rows.len() as i32, Self::SqlRow(env)?, JObject::null()) {
            Ok(r) => r,
            Err(e) => return Err(format!("Failed to create dev.array21.jdbd.datatypes.SqlRow[]: {:?}", e))
        };

        let mut index = 0;
        for row in rows.into_iter() {
            match env.set_object_array_element(array, index, row) {
                Ok(_) => {},
                Err(e) => return Err(format!("Failed to set element in dev.array21.jdbd.datatypes.SqlRow[]: {:?}", e))
            }
            index += 1;
        }

        Ok(JObject::from(array))
    }
}