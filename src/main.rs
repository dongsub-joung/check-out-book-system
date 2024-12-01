mod book;
mod cli;
fn main() {
    // DB conn
    
    
    // new obj with CRUD
    let a_book= book::Book::new(
        "title".to_string()
        , "autho".to_string()
        , 5
    );


    // CLI interface
    cli::logo(); 
}

// @todo ORM diesel
// use tokio_postgres::{NoTls, Error};

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     // Connection string to your PostgreSQL database
//     let connection_string = "host=localhost user=postgres password=yourpassword dbname=yourdatabase";

//     // Connect to the database
//     let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;

//     // Spawn a separate task to manage the connection.
//     tokio::spawn(async move {
//         if let Err(e) = connection.await {
//             eprintln!("Connection error: {}", e);
//         }
//     });

//     // Create a table
//     client
//         .execute(
//             "CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY, name TEXT NOT NULL)",
//             &[],
//         )
//         .await?;

//     // Insert a row into the table
//     client
//         .execute(
//             "INSERT INTO users (name) VALUES ($1)",
//             &[&"Alice"],
//         )
//         .await?;

//     // Query the table
//     let rows = client
//         .query("SELECT id, name FROM users", &[])
//         .await?;

//     // Print the results
//     for row in rows {
//         let id: i32 = row.get(0);
//         let name: &str = row.get(1);

//         println!("Found user: {} with ID: {}", name, id);
//     }

//     Ok(())
// }
