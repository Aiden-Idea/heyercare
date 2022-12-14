use rusqlite::{params, Connection, Result};

pub struct Repository {
    sql_conn: rusqlite::Connection
}

impl Repository {
    pub fn new() -> Self {
        Repository { 
            sql_conn: rusqlite::Connection::open("./src/db/heyercare.db").unwrap()
        }
    }

    // 插入数据
    pub fn insert_data(&self, sql: &str) {
        match self.sql_conn.execute(sql, []) {
            Ok(o) => o,
            Err(e) => {panic!("{:?}", e)}
        };
    }
}


pub fn init_db() -> Result<()> {
    let database_file = "./src/db/heyercare.db";
    let conn = Connection::open(database_file)?;

    // 初始化时就判断所有表在不在 如果没有表 那么创建 ，如果有表 跳过

    // let flag1 = check_table_existed("table_1", &conn);
    // create_table(&conn);

    // if !check_person_existed("Tester1", &con) {
    //     insert_person("Tester1", &con);
    // }

    // insert_data(&conn);

    // get_data(&conn);

    Ok(())
}

// 创建表
pub fn create_table(con: &Connection) {
    let sql : &str = "CREATE TABLE IF NOT EXISTS `patients`(
        `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, 
        `name` TEXT NOT NULL,
        `age` INTEGER  NOT NULL
        );";
    
    match con.execute(sql, params![]) {
        Ok(o) => o,
        Err(e) => panic!("{:?}", e),
    };
}

// 判断表是否存在
pub fn check_table_existed(table_name: &str, con: &Connection) -> bool {
    let _sql: &str =
        "SELECT COUNT(`name`) FROM `sqlite_master` WHERE `type` = 'table' AND `name` = ?";
    println!("_sql: {}", _sql);    
    let mut stmt = con.prepare(_sql).unwrap();
    let rs = stmt.query_row([table_name], |row| {
        return row.get(0) as Result<i32>;
    });
    let count = rs.unwrap();
    return count > 0;
}



// pub fn get_data(conn: &Connection) -> Result<()> {
//     let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
//     let person_iter = stmt.query_map([], |row| {
//         Ok(Person {
//             id: row.get(0)?,
//             name: row.get(1)?,
//         })
//     })?;

//     for person in person_iter {
//         println!("Found person {:?}", person.unwrap())
//     }
//     Ok(())
// }

pub fn check_person_existed(person: &str, con: &Connection) -> bool {
    let sql: &str = "SELECT COUNT(`Id`) FROM `table_1` WHERE `Name` = ?";
    let mut stmt = con.prepare(sql).unwrap();
    let rs = stmt.query_row(params![person], |row| row.get(0) as Result<i32>);
    let count = rs.unwrap();
    return count > 0;
}
