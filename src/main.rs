mod oxide_lib;

use crate::oxide_lib::di::*;
use oxide_macro::{bean_provider, Component};
use rusqlite::fallible_iterator::FallibleIterator;
use rusqlite::{Connection, Row};
use std::io::stdin;

#[derive(Component)]
struct UserService {
    #[wired]
    dao: &'static Dao,
}

impl UserService {
    fn start(&self) {
        loop {
            let x = input();
            match x.as_str() {
                "create" => {
                    self.dao.create_rand_user();
                }
                "all" => {
                    let users = self.dao.get_all();
                    println!("all users{:?}", users);
                }
                "single" => {
                    if let Some(user) = self.dao.get_single() {
                        println!("some random user: {:?}", user)
                    } else {
                        println!("some error occured while executing query :(")
                    };
                }
                _ => break,
            }
        }
    }
}

#[bean_provider]
fn conn_provider() -> Connection {
    let c = Connection::open("storage.sqlite").unwrap();
    let _ = c.execute("DROP TABLE IF EXISTS user", []);
    let _ = c
        .execute(
            "CREATE TABLE IF NOT EXISTS user(id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL , name TEXT)",
            (),
        )
        .unwrap();
    // здесь будет какая то инициализация например, но вообще мы потом это спихнём на раннеры/стартеры
    // плюс добавим простенький механизм для исполнения каких либо действий, по типу
    // спрингового pre-build/post-destroy
    c
}

#[derive(Component)]
struct Dao {
    #[wired]
    conn: &'static Connection,
}

impl Dao {
    fn create_rand_user(&self) {
        self.conn
            .execute(
                "INSERT INTO user(name) VALUES (?1)",
                [rand::random::<u32>().to_string()],
            )
            .unwrap();

        println!("inserted");
    }

    fn get_all(&self) -> Vec<User> {
        let mut stmt = self.conn.prepare("SELECT id, name from user").unwrap();
        let rows = stmt.query([]).unwrap();
        let users = rows
            .map(|it| User::try_from(it))
            .collect::<Vec<_>>()
            .unwrap();
        users
    }

    fn get_single(&self) -> Option<User> {
        self.conn
            .query_one("SELECT id, name from user limit 1", (), |it| {
                User::try_from(it)
            })
            .ok()
    }
}
#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
}

impl TryFrom<&Row<'_>> for User {
    type Error = rusqlite::Error;

    fn try_from(v: &Row<'_>) -> Result<Self, Self::Error> {
        let id = v.get("id")?;
        let name = v.get("name")?;
        Ok(User { id, name })
    }
}

fn main() {
    let mut ctx = Context::new();
    ctx.init();
    let service = ctx.get::<UserService>().unwrap();
    service.start();
}

fn input() -> String {
    let mut s = String::default();
    stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}
