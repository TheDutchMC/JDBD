use std::error::Error;
use jni::objects::JObject;
use jni::JNIEnv;
use jni::sys::jobjectArray;
use mysql::Value;
use postgres::types::{IsNull, ToSql, Type};
use postgres::types::private::BytesMut;

pub type SqlParameterArray = jobjectArray;

#[derive(Debug)]
pub enum SqlParameter {
    Null,
    Bytes(Vec<u8>),
    Int(i64),
    Float(f32),
    Double(f64),
}

impl Into<mysql::Value> for SqlParameter {
    fn into(self) -> Value {
        match self {
            Self::Null => mysql::Value::NULL,
            Self::Int(x) => mysql::Value::Int(x),
            Self::Bytes(x) => mysql::Value::Bytes(x),
            Self::Float(x) => mysql::Value::Float(x),
            Self::Double(x) => mysql::Value::Double(x),
        }
    }
}

impl ToSql for SqlParameter {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> where Self: Sized {
        match self {
            Self::Null => Option::<i64>::None.to_sql(ty, out),
            Self::Int(x) => x.to_sql(ty, out),
            Self::Bytes(x) => x.to_sql(ty, out),
            Self::Float(x) => x.to_sql(ty, out),
            Self::Double(x) => x.to_sql(ty, out),
        }
    }

    fn accepts(_: &Type) -> bool where Self: Sized {
        true
    }

    fn to_sql_checked(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.to_sql(ty, out)
    }
}

pub fn into_sql_parameter(env: &JNIEnv<'_>, param: JObject) -> Result<SqlParameter, jni::errors::Error> {
    let param_type = env.get_field(param, "type", "Ldev/array21/jdbd/datatypes/SqlParameterType;")?.l()?;
    let param_type_string_object = env.call_method(param_type, "toString", "()Ljava/lang/String;", &[])?.l()?;
    let param_type_string: String = env.get_string(param_type_string_object.into())?.into();

    let result = match param_type_string.as_str() {
        "NULL" => {
            SqlParameter::Null
        },
        "BYTES" => {
            let bytes_object = env.get_field(param, "bytesVal", "[B")?.l()?;
            let bytes_len = env.get_array_length(bytes_object.into_inner())?;
            let mut buf = vec![0i8; bytes_len as usize];
            env.get_byte_array_region(bytes_object.into_inner(), 0, &mut buf)?;

            let buf = buf.into_iter()
                .map(|x| x as u8)
                .collect::<Vec<_>>();

            SqlParameter::Bytes(buf)
        },
        "INT" => {
            let int_object = env.get_field(param, "longVal", "J")?.j()?;
            SqlParameter::Int(int_object)
        },
        "FLOAT" => {
            let float_object = env.get_field(param, "floatVal", "F")?.f()?;
            SqlParameter::Float(float_object)
        },
        "DOUBLE" => {
            let double_object = env.get_field(param, "doubleVal", "D")?.d()?;
            SqlParameter::Double(double_object)
        },
        _ => panic!("Invalid sql parameter type {}", &param_type_string)
    };

    Ok(result)
}