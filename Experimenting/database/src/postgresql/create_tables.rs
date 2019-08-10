extern crate postgres;

use postgres::{Connection, TlsMode, Error};

fn main() -> Result<(), Error> {
    let conn = Connection::connect("postgresql://postgres:postgres@localhost/library", 
                                    TlsMode::None)?;
    
     conn.execute("CREATE TABLE IF NOT EXISTS author (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    country         VARCHAR NOT NULL
                  )", &[])?;

    conn.execute("CREATE TABLE IF NOT EXISTS book  (
                    id              SERIAL PRIMARY KEY,
                    title           VARCHAR NOT NULL,
                    author_id       INTEGER NOT NULL REFERENCES author
                )", &[])?;

    Ok(())

}
