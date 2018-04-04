extern crate build_entity;

use build_entity::*;
use self::models::*;

#[macro_use]
extern crate tera;
use tera::Context;

extern crate diesel;
use diesel::*;

use std::env;

use std::fs;
use std::io::{BufWriter, Write};

extern crate inflector;
use inflector::Inflector;

fn main() {
    let args: Vec<String> = env::args().collect();
    let table_schema = &args[1];
    let table_name = &args[2];
    let query = format!(r#"
        select
            table_schema,
            table_name,
            column_name,
            ordinal_position,
            data_type,
            is_nullable
        from
            information_schema.columns as columns
        where
            table_schema = '{1}'
        and table_name = '{0}'
    "#, table_name, table_schema);
    println!("{}", &query);

    let connection = establish_connection();
    let results = sql_query(query)
            .load::<Columns>(&connection);

    let tera = compile_templates!("templates/*");
    let mut context = Context::new();
    let mut column_list: Vec<Columns4Tera> = vec![];
    match results {
        Ok(column_info_list) => {
            context.add("table_name", &table_name);
            for column_info in column_info_list {
                let column_name_camel = column_info.column_name.to_camel_case();
                let data_type = match &*column_info.data_type {
                    "varchar" => "String",
                    "int" => "Int",
                    "datetime" => "Date",
                    _ => "unknown"
                };

                let column = Columns4Tera::new(
                    column_info.column_name,
                    column_name_camel,
                    data_type.to_string(),
                    column_info.is_nullable);
                if &column.columnName == "created_at" || &column.columnName == "updated_at" {

                } else {
                    column_list.push(column);
                }
            }
            context.add("column_list", &column_list);

            let model_name = format!("{0}s", &table_name.to_class_case());
            context.add("ModelName", &model_name);

            let entity_name = format!("{0}Entity", &table_name.to_class_case());
            context.add("EntityName", &entity_name);

            context.add("camelCaseName", &table_name.to_camel_case());
            let file_name = "TemplateEntity.scala.tpl";
            match tera.render(&file_name, &context) {
                Ok(content) => {
                    let file_name = format!("products/{0}Entity.scala", &table_name.to_class_case());
                    let mut f = BufWriter::new(fs::File::create(file_name).unwrap());
                    f.write(content.as_bytes()).unwrap();
                    // println!("{:?}", content)
                },
                Err(err) => println!("{:?}", err)
            }
        },
        Err(err) => println!("{:?}", err)
    }
}
