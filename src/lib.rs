

pub fn insert(table: &String, fields: &Vec<String>) -> String {

    let cols = fields.join(" , ");

    let mut nos: Vec<String> = vec![]; 
    for x in 1..(fields.len() + 1) {

        let parameterized = format!("${}",x.to_string());

        nos.push(parameterized)
    }
    
    let values = nos.join(" , ");

    format!("insert into {} ({}) values ({})", table, cols, values)
}

pub fn update(table: &String, fields: &Vec<String>) -> String {

    let mut col_update = vec![];
    for x in 1..(fields.len() + 1) {

        let field = fields.get(x - 1).unwrap();

        col_update.push(format!("{} = ${}",field,x))
    }

    format!("update {} set {} where id = ${}",table,col_update.join(" , "),fields.len() + 1)
}

pub fn select(table: &String) -> String {

    format!("select * from {} where id = $1",table)
}


pub fn select_all(table: &String) -> String {

    format!("select * from {}", table)
}

pub fn delete(table: &String) -> String {

    format!("delete from {} where id = $1",table)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn insert_works(){

        let table = String::from("widgets");

        let fields = 
            vec![
                String::from("id"),
                String::from("name")
            ];

        let statement = super::insert(&table,&fields);

        assert_eq!("insert into widgets (id , name) values ($1 , $2)",statement);
    }

    #[test]
    fn update_works(){

        let table = String::from("widgets");

        let fields = 
            vec![
                String::from("name"),
                String::from("slug"),
                String::from("width")
            ];

        let statement = super::update(&table,&fields);

        println!("{}", &statement);

        assert_eq!("update widgets set name = $1 , slug = $2 , width = $3 where id = $4",statement);
    }

    #[test]
    fn delete_works(){

        let statement = super::delete(&"widgets".to_string());

        assert_eq!("delete from widgets where id = $1",statement)
    }

    #[test]
    fn select_all_works(){

        let statement = super::select_all(&"widgets".to_string());

        assert_eq!("select * from widgets",statement)
    }

    #[test]
    fn select_works(){

        let statement = super::select(&"widgets".to_string());

        assert_eq!("select * from widgets where id = $1",statement)
    }
}
