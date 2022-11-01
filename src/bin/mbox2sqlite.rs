use clap::Parser;
use std::fmt::Debug;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use mail_parser::Message;
use rusqlite::Connection;
use mbox2sqlite::email::Email;
use mbox2sqlite::mbox::Mbox;
use mbox2sqlite::sqlite::DbContext;

#[derive(Parser, Debug)]
#[command(author, version, about="Convert an mbox file into an SQLite database.")]
struct Args {
    #[arg(
        short,
        long,
        default_value = "mbox.sqlite",
        help = "SQLite database to write emails to."
    )]
    database: PathBuf,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Wipe all existing emails in the database before importing new ones."
    )]
    wipe: bool,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Don't print progress updates."
    )]
    quiet: bool,
    mbox: PathBuf
}

fn main() {
    let args = Args::parse();
    let mut mbox = match Mbox::open(&args.mbox) {
        Ok(mbox) => mbox,
        Err(_) => {
            println!("unable to open '{}'", args.mbox.to_str().unwrap());
            exit(1)
        }
    };
    let conn = match Connection::open(args.database) {
        Ok(conn) => conn,
        Err(error) => {
            println!("{}", error.to_string());
            exit(1)
        }
    };
    let mut ctx = DbContext::prepare(&conn, args.wipe);
    process_messages(&mut ctx, &mut mbox, args.quiet);
}

fn process_messages(ctx: &mut DbContext, mbox: &mut Mbox, quiet: bool) {
    ctx.begin_transaction();
    let mut skipped_messages = 0;
    let mut inserted_messages = 0;
    let mut processed_messages = 0;
    for raw_message in mbox {
        let msg = Message::parse(raw_message.as_bytes());
        match msg {
            None => {
                skipped_messages += 1;
            }
            Some(msg) => {
                let email_id = ctx.insert_email(&Email::from(&msg));
                for attachment in msg.get_attachments() {
                    ctx.insert_attachment(email_id, attachment.get_contents());
                }
                inserted_messages += 1;
            }
        }
        processed_messages += 1;
        if !quiet && processed_messages % 100 == 0 {
            print!("\rMessages processed: {}", processed_messages);
            io::stdout().flush().unwrap();
        }
    }
    ctx.commit();
    if !quiet {
        println!("\rMessages processed: {}", processed_messages);
        println!("\rMessages inserted: {}", inserted_messages);
        println!("\rMessages skipped: {}", skipped_messages);
    }
}
