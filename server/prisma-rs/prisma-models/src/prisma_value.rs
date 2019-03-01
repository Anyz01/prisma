pub enum GraphqlId {
    String(String),
    Int(usize),
}

pub enum PrismaValue {
    String(String),
    Float(f32),
    Boolean(bool),
    DateTime(String),
    Enum(String),
    Json(String),
    Int(i32),
    Relation(usize),
    Null,
    Uuid(String),
    GraphqlId(GraphqlId),
}

impl Into<DatabaseValue> for PrismaValue {
    fn into(self) -> DatabaseValue {
        match self {
            PrismaValue::String(s) => s.into(),
            PrismaValue::Float(f) => (f as f64).into(),
            PrismaValue::Boolean(b) => b.into(),
            PrismaValue::DateTime(d) => d.into(),
            PrismaValue::Enum(e) => e.into(),
            PrismaValue::Json(j) => j.into(),
            PrismaValue::Int(i) => (i as i64).into(),
            PrismaValue::Relation(i) => i.into(),
            PrismaValue::Null => DatabaseValue::Parameterized(ParameterizedValue::Null),
            PrismaValue::Uuid(u) => u.into(),
            PrismaValue::GraphqlId(id) => {
                GraphqlId::String(s) => s.into(),
                GraphqlId::Int(i) => i.into()
            }
        }
    }
}
