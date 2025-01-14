use rusqlite::{Connection, Result};

pub struct DB {
    conn: Connection,
}

#[derive(Debug)]
pub struct DbBlock {
    pub height: u64,
    pub hash: String,
}

impl DB {
    pub fn new() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        Ok(Self { conn })
    }

    pub fn initialize(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS blocks (height INTEGER PRIMARY KEY AUTOINCREMENT, hash TEXT)",
            [],
        )?;
        Ok(())
    }

    pub fn get_next_block_height(&self) -> Result<u64> {
        let query = "SELECT height FROM blocks ORDER BY height DESC LIMIT 1";
        let mut stmt = self.conn.prepare(query)?;
        let height = stmt.query_row([], |row| row.get(0));
        Ok(height.unwrap_or(0) + 1)
    }

    pub fn insert_block(&self, block: DbBlock) -> Result<()> {
        self.conn.execute(
            "INSERT INTO blocks (hash) VALUES (?)",
            [block.hash],
        )?;
        Ok(())
    }
}
