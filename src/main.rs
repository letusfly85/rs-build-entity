extern crate build_entity;

use build_entity::*;
use self::models::*;

#[macro_use]
extern crate tera;
use tera::Tera;
use tera::Context;

extern crate diesel;
use diesel::*;

use std::env;

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
    let mut column_list: Vec<String> = vec![];
    match results {
        Ok(column_info_list) => {
            context.add("table_name", &table_name);
            for column_info in column_info_list {
                println!("{:?}", column_info);
                context.add("column_info", &column_info);
                column_list.push(column_info.column_name);
            }
            context.add("column_list", &column_list);
            let file_name = "TemplateEntity.scala";
            match tera.render(&file_name, &context) {
                Ok(content) => println!("{:?}", content),
                Err(err) => println!("{:?}", err)
            }
        },
        Err(err) => println!("{:?}", err)
    }
}
