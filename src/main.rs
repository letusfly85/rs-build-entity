extern crate build_entity;

use build_entity::*;
use self::models::*;

#[macro_use]
extern crate tera;
use tera::Context;

extern crate dotenv;
use dotenv::dotenv;

extern crate diesel;
use diesel::*;

use std::env;

extern crate regex;
use regex::Regex;

use std::fs;
use std::io::{BufWriter, Write};

extern crate inflector;
use inflector::Inflector;

fn main() {
    dotenv().ok();
    let re = Regex::new(r"([\w]+)://([a-z:]+)@0.0.0.0:([0-9]{4})/([a-z0-9_]+)").unwrap();
    let database_url = env::var("DATABASE_URL").unwrap();
    let caps = re.captures(&database_url).unwrap();
    let database_name = caps.get(4).unwrap().as_str();
    println!("database: {}", &database_name);
    let args: Vec<String> = env::args().collect();
    let table_name = &args[1];
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
    "#, table_name, database_name);
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
                    "tinyint" => "Boolean",
                    _ => "unknown"
                };

                let column = Columns4Tera::new(
                    column_info.column_name,
                    column_name_camel,
                    data_type.to_string(),
                    column_info.is_nullable);
                if &column.column_name == "created_at" || &column.column_name == "updated_at" {

                } else {
                    column_list.push(column);
                }
            }
            context.add("column_list_length", &column_list.len());
            context.add("column_list", &column_list);

            let model_name = format!("{0}s", &table_name.to_class_case());
            context.add("ModelName", &model_name);

            let entity_name = format!("{0}Entity", &table_name.to_class_case());
            context.add("EntityName", &entity_name);

            context.add("camelCaseName", &table_name.to_camel_case());
            let file_name = "TemplateEntity.scala.tpl";
            match tera.render(&file_name, &context) {
                Ok(content) => {
                    let file_name = format!("products/{0}.scala", &entity_name);
                    let mut f = BufWriter::new(fs::File::create(file_name).unwrap());
                    f.write(content.as_bytes()).unwrap();
                },
                Err(err) => println!("{:?}", err)
            }
            let repository_name = format!("{0}Repository", &table_name.to_class_case());
            let table_alias = generate_table_alias(table_name.to_string());
            context.add("TableAlias", &table_alias);
            context.add("RepositoryName", &repository_name);
            let file_name = "TemplateRepository.scala.tpl";
            match tera.render(&file_name, &context) {
                Ok(content) => {
                    let file_name = format!("products/{0}.scala", &repository_name);
                    let mut f = BufWriter::new(fs::File::create(file_name).unwrap());
                    f.write(content.as_bytes()).unwrap();
                },
                Err(err) => println!("{:?}", err)
            }
            let akka_http_entity_name = format!("{0}EntityOnAkkaHttp", &table_name.to_class_case());
            context.add("EntityOnAkkaHttp", &akka_http_entity_name);
            let file_name = "EntityOnAkkaHttp.scala.tpl";
            match tera.render(&file_name, &context) {
                Ok(content) => {
                    let file_name = format!("products/{0}.scala", &akka_http_entity_name);
                    let mut f = BufWriter::new(fs::File::create(file_name).unwrap());
                    f.write(content.as_bytes()).unwrap();
                },
                Err(err) => println!("{:?}", err)
            }
        },
        Err(err) => println!("{:?}", err)
    }
}

fn generate_table_alias(table_name: String) -> String {
    let table_prefix_list: Vec<&str> = table_name.split("_").collect();
    let prefix_list: Vec<&str> = table_prefix_list.iter().map(|&table_prefix| &table_prefix[0..1]).collect();
    println!("{} -> {}", &table_name, prefix_list.join(""));

    return prefix_list.join("").to_string();
}
