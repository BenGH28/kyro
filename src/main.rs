struct Chapter {
	number: u32,
	verses: u32,
}

struct Book {
	chapters: Vec<Chapter>,
}

struct Bible {
	books: Vec<Book>,
}

fn main() {
	println!("Hello, world!");
}
