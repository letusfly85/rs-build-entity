table! {
    columns(table_schema, table_name, ordinal_position) {
        table_schema -> Varchar,
        table_name -> Varchar,
        column_name -> Varchar,
        ordinal_position -> BigInt,
        data_type -> Varchar,
    }
}

use diesel::sql_types::{Text, BigInt};

#[derive(Debug, QueryableByName, Serialize)]
pub struct Columns {
    #[sql_type="Text"]
    pub table_schema: String,
    #[sql_type="Text"]
    pub table_name: String,
    #[sql_type="Text"]
    pub column_name: String,
    #[sql_type="BigInt"]
    pub ordinal_position: i64,
    #[sql_type="Text"]
    pub data_type: String,
    #[sql_type="Text"]
    pub is_nullable: String,
}

#[derive(Serialize)]
pub struct Columns4Tera {
    pub column_name: String,
    pub column_name_camel: String,
    data_type: String,
    is_nullable: String
}

impl Columns4Tera {
    pub fn new(column_name:String, column_name_camel: String, data_type: String, is_nullable: String) -> Columns4Tera {
        Columns4Tera{
            column_name,
            column_name_camel,
            data_type,
            is_nullable
        }
    }
}
