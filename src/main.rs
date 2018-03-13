extern crate build_entity;

use build_entity::*;
use self::models::*;

// #[macro_use]
// extern crate tera;

extern crate diesel;
use diesel::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let table_name = &args[1];
    println!("{:?}", table_name);

    let connection = establish_connection();

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
    let results = sql_query(query)
            .load::<Columns>(&connection);

    match results {
        Ok(column_info_list) =>
            for column_info in column_info_list {
                println!("{:?}", column_info)
            },
        Err(err) => println!("{:?}", err)
    }
}
