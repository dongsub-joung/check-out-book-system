mod book;
mod cli;
fn main() {
    // @todo ORM diesel
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

