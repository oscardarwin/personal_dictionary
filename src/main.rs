use inquire::{
    error::{CustomUserError, InquireResult},
    required, CustomType, MultiSelect, Select, Text,
};
use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use typedb_driver::{
    Connection, DatabaseManager, Error, Promise, Session, SessionType, TransactionType,
};

fn main() -> InquireResult<()> {
    loop {
        let commands = Command::iter().collect();
        match Select::new("Pick Action:", commands).prompt()? {
            Command::Add => println!("Adding."),
            Command::Setup => setup_db().expect("DB ERROR"),
            Command::Exit => {
                break;
            }
        }
    }

    Ok(())
}

#[derive(EnumIter)]
enum Command {
    Add,
    Setup,
    Exit,
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let name = match self {
            Command::Add => "add",
            Command::Setup => "setup",
            Command::Exit => "exit",
        };

        write!(f, "{}", name)
    }
}

fn setup_db() -> Result<(), Error> {
    const DB_NAME: &str = "access-management-db";
    const SERVER_ADDR: &str = "127.0.0.1:1729";
    let driver = Connection::new_core(SERVER_ADDR)?;
    let databases = DatabaseManager::new(driver);
    if databases.contains(DB_NAME)? {
        println!("recreating database");
        databases.get(DB_NAME)?.delete()?;
    }
    databases.create(DB_NAME)?;
    {
        let session = Session::new(databases.get(DB_NAME)?, SessionType::Schema)?;
        let tx = session.transaction(TransactionType::Write)?;
        tx.query().define("define person sub entity;").resolve()?;
        tx.query()
            .define("define name sub attribute, value string; person owns name;")
            .resolve()?;
        tx.commit().resolve()?;
    }
    Ok(())
}
