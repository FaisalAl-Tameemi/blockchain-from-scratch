use tokio_rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct DB {
    conn: Connection,
}

#[derive(Debug)]
pub struct DbBlock {
    pub height: u64,
    pub hash: String,
}

impl DB {
    pub async fn new() -> Result<Self> {
        let conn = Connection::open_in_memory().await?;
        Ok(Self { conn })
    }

    pub async fn initialize(&self) -> Result<()> {
        self.conn.call(|conn| {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS blocks (height INTEGER PRIMARY KEY AUTOINCREMENT, hash TEXT)",
                [],
            )?;
            Ok(())
        }).await?;
        Ok(())
    }

    pub async fn get_next_block_height(&self) -> Result<u64> {
        let query = "SELECT height FROM blocks ORDER BY height DESC LIMIT 1";
        let result = self.conn.call(|conn| {
            let mut stmt = conn.prepare(query)?;
            let height = stmt.query_row([], |row| row.get(0));
            Ok(height.unwrap_or(0) + 1)
        }).await?;
        Ok(result)
    }

    pub async fn insert_block(&self, block: DbBlock) -> Result<()> {
        self.conn.call(|conn| {
            conn.execute(
                "INSERT INTO blocks (hash) VALUES (?)",
                [block.hash],
            )?;
            Ok(())
        }).await?;
        Ok(())
    }
}
