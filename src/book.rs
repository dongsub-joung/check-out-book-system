pub struct Book{
    // postgres id
    id: i32,
    title: String,
    autho: String,
    volume: i32,
}

impl Book{
    pub fn new(title: String, autho: String, volume: i32)-> Book{
        Book{ id: 1, title: title, autho: autho, volume: volume }
    }
}