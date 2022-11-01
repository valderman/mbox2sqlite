use rusqlite::{Connection, params, Statement};
use crate::email::Email;

pub struct DbContext<'a> {
    connection: &'a Connection,
    insert_email_statement: Statement<'a>,
    insert_attachment_statement: Statement<'a>
}

impl <'a> DbContext<'a> {
    pub fn prepare(conn: &Connection, wipe: bool) -> DbContext {
        if wipe {
            drop_tables(conn);
        }
        create_tables(conn);

        let insert_email_stmt = conn.prepare("
            INSERT INTO emails (
                timestamp,
                sender,
                recipient,
                cc,
                bcc,
                subject,
                text_body,
                html_body,
                gmail_labels
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            RETURNING id;
        ").unwrap();
        let insert_attachment_stmt = conn.prepare("
            INSERT INTO attachments (email_id, attachment)
            VALUES (?1, ?2);
        ").unwrap();
        DbContext {
            connection: conn,
            insert_email_statement: insert_email_stmt,
            insert_attachment_statement: insert_attachment_stmt
        }
    }

    pub fn begin_transaction(&self) {
        self.connection.execute("BEGIN TRANSACTION;", ()).unwrap();
    }

    pub fn commit(&self) {
        self.connection.execute("COMMIT;", ()).unwrap();
    }

    pub fn insert_email(&mut self, msg: &Email) -> u32 {
        let mut rows = self.insert_email_statement
            .query(params![
                msg.timestamp,
                msg.from,
                msg.to,
                msg.cc,
                msg.bcc,
                msg.subject,
                msg.text_body,
                msg.html_body,
                msg.gmail_labels
            ])
            .unwrap();
        rows.next().unwrap().unwrap().get_unwrap(0)
    }

    pub fn insert_attachment(&mut self, email_id: u32, attachment: &[u8]) {
        self.insert_attachment_statement.execute(params![email_id, attachment]).unwrap();
    }
}

fn drop_tables(conn: &Connection) {
    conn.execute("DROP TABLE IF EXISTS attachments;", ()).unwrap();
    conn.execute("DROP TABLE IF EXISTS emails;", ()).unwrap();
}

fn create_tables(conn: &Connection) {
    conn.execute("
            CREATE TABLE IF NOT EXISTS emails (
                id INTEGER PRIMARY KEY,
                timestamp TEXT NULL,
                sender TEXT NULL,
                recipient TEXT NULL,
                cc TEXT NULL,
                bcc TEXT NULL,
                subject TEXT NOT NULL,
                text_body TEXT NOT NULL,
                html_body TEXT NOT NULL,
                gmail_labels TEXT NULL
            );", ()
    ).unwrap();
    conn.execute("
            CREATE TABLE IF NOT EXISTS attachments (
                id INTEGER PRIMARY KEY,
                email_id INTEGER REFERENCES emails (id),
                attachment BLOB NOT NULL
            );", ()
    ).unwrap();
}
