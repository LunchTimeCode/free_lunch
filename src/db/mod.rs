use libsql::Database;

pub fn db_health(db: &Database) -> bool {
    db.connect().is_ok()
}
