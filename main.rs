// TASK 4

fn double_in_place(values: &mut [i32]) {
    for val in values.iter_mut() {
        *val *= 2;
    }
}

fn largest(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

fn remove_duplicates(v: &mut Vec<i32>) {
    let mut seen = std::collections::HashSet::new();
    v.retain(|e| seen.insert(*e));
}

fn running_sum(v: &[i32]) -> Vec<i32> {
    let mut sum = 0;
    v.iter().map(|&x| {
        sum += x;
        sum
    }).collect()
}

fn task4_vectors() {
    let mut v = vec![3, 1, 3, 2, 1];
    remove_duplicates(&mut v);
    double_in_place(&mut v);

    println!("v = {:?}", v);
    println!("running_sum = {:?}", running_sum(&v));
    println!("largest = {:?}", largest(&v));
}

// TASK 5 — Structs and methods

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn contains(&self, p: Point) -> bool {
        p.x >= self.top_left.x
            && p.x <= self.top_left.x + self.width
            && p.y >= self.top_left.y
            && p.y <= self.top_left.y + self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn task5_structs() {
    let mut rect1 = Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        width: 10.0,
        height: 5.0,
    };

    let rect2 = Rectangle {
        top_left: Point { x: 1.0, y: 1.0 },
        width: 4.0,
        height: 2.0,
    };

    let p = Point { x: 3.0, y: 2.0 };

    println!("Area: {}", rect1.area());
    println!("Contains Point: {}", rect1.contains(p));
    println!("Can hold rect2: {}", rect1.can_hold(&rect2));

    rect1.scale(2.0);
    println!("Scaled area: {}", rect1.area());
}

// TASK 6 — A library

#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    year: u16,
}

impl Book {
    fn new(title: &str, author: &str, year: u16) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            year,
        }
    }
}

#[derive(Debug)]
struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Self {
        Library { books: Vec::new() }
    }

    fn add(&mut self, book: Book) {
        self.books.push(book);
    }

    fn titles(&self) -> Vec<&str> {
        self.books.iter().map(|b| b.title.as_str()).collect()
    }

    fn by_author(&self, author: &str) -> Vec<&Book> {
        self.books.iter().filter(|b| b.author == author).collect()
    }

    fn oldest(&self) -> Option<&Book> {
        self.books.iter().min_by_key(|b| b.year)
    }

    fn remove(&mut self, title: &str) -> Option<Book> {
        if let Some(index) = self.books.iter().position(|b| b.title == title) {
            Some(self.books.remove(index))
        } else {
            None
        }
    }
}

fn task6_library() {
    let mut lib = Library::new();

    lib.add(Book::new("1984", "George Orwell", 1949));
    lib.add(Book::new("Animal Farm", "George Orwell", 1945));
    lib.add(Book::new("The Hobbit", "J.R.R. Tolkien", 1937));

    println!("Titles: {:?}", lib.titles());
    println!("By Author (George Orwell): {:?}", lib.by_author("George Orwell"));
    println!("Oldest Book: {:?}", lib.oldest());

    let removed = lib.remove("1984");
    println!("Removed: {:?}", removed);
    println!("Titles after removal: {:?}", lib.titles());
}

// MAIN FUNCTION

fn main() {
    println!("--- TASK 4 ---");
    task4_vectors();

    println!("\n--- TASK 5 ---");
    task5_structs();

    println!("\n--- TASK 6 ---");
    task6_library();
}