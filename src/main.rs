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
    let table_name = &args[1];
    let query = format!(r#"
        select
            table_schema,
            table_name,
            column_name,
            ordinal_position,
            data_type
        from
            information_schema.columns as columns
        where
            table_name = '{0}'
    "#, table_name);
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
                    _ => "unknown"
                };

                let column = Columns4Tera::new(
                    column_info.column_name,
                    column_name_camel,
                    data_type.to_string());
                column_list.push(column);
            }
            context.add("column_list", &column_list);
            context.add("entityName", &table_name.to_class_case());
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
