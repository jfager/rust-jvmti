use super::native::JavaClass;

///
/// Enumeration of the possible Java types.
///
#[derive(Debug, Eq, PartialEq)]
pub enum JavaType<'a> {
    Boolean,
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Void,
    Class(&'a str),
    Array(Box<JavaType<'a>>)
}

impl<'a> JavaType<'a> {

    /// Convert a given type signature into a JavaType instance (if possible). None is returned
    /// if the conversation was not successful.
    pub fn parse(signature: &'a str) -> Option<JavaType<'a>> {
        match signature.len() {
            0 => None,
            1 => match &*signature {
                "B" => Some(JavaType::Byte),
                "C" => Some(JavaType::Char),
                "D" => Some(JavaType::Double),
                "F" => Some(JavaType::Float),
                "I" => Some(JavaType::Int),
                "L" => Some(JavaType::Long),
                "S" => Some(JavaType::Short),
                "V" => Some(JavaType::Void),
                "Z" => Some(JavaType::Boolean),
                _ => None
            },
            _ => {
                match signature.chars().nth(0).unwrap() {
                    '[' => {
                        let (_, local_type) = signature.split_at(1);

                        match JavaType::parse(local_type) {
                            Some(result) => Some(JavaType::Array(Box::new(result))),
                            None => None
                        }
                    },
                    'L' => Some(JavaType::Class(signature)),
                    _ => None
                }
            }
        }
    }

    ///
    /// Converts the given Java type into a conventional human-readable representation
    ///
    pub fn to_string(java_type: &JavaType) -> String {
        match *java_type {
            JavaType::Byte => "byte".to_string(),
            JavaType::Char => "char".to_string(),
            JavaType::Double => "double".to_string(),
            JavaType::Float => "float".to_string(),
            JavaType::Int => "int".to_string(),
            JavaType::Long => "long".to_string(),
            JavaType::Short => "short".to_string(),
            JavaType::Void => "void".to_string(),
            JavaType::Boolean => "boolean".to_string(),
            JavaType::Array(ref inner_type) => format!("{}[]", JavaType::to_string(inner_type)),
            JavaType::Class(cls) => cls.trim_left_matches("L").trim_right_matches(";").replace(";", "").replace("/", ".").to_string()
        }
    }
}

///
/// Represents a JNI local reference to a Java class
///
pub struct ClassId {
    pub native_id: JavaClass
}

pub struct Class<'a> {
    pub id: ClassId,
    pub signature: JavaType<'a>
}

impl<'a> Class<'a> {

    /// Constructs a new Class instance.
    pub fn new(id: ClassId, signature: JavaType<'a>) -> Class {
        Class { id: id, signature: signature }
    }
}
