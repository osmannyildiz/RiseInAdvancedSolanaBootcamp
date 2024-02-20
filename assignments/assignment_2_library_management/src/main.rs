enum Publication {
    Book(Book),
    Magazine(Magazine),
}

struct Book {
    title: String,
    author: String,
    page_count: u32,
}

struct Magazine {
    title: String,
    issue: u32,
    topic: String,
}

fn print_publications(publications: Vec<Publication>) {
    for publication in publications {
        match publication {
            Publication::Book(ref book) => {
                println!(
                    "== KİTAP (adı: {}, yazar: {}, {} sayfa) ==",
                    book.title, book.author, book.page_count
                );
            }
            Publication::Magazine(ref magazine) => {
                println!(
                    "== DERGİ (adı: {}, sayı: {}, konu: {}) ==",
                    magazine.title, magazine.issue, magazine.topic
                );
            }
        }
    }
}

fn main() {
    let book1 = Book {
        title: "Her Yönüyle Python".to_string(),
        author: "Fırat Özgül".to_string(),
        page_count: 736,
    };
    let book2 = Book {
        title: "C# Eğitim Kitabı".to_string(),
        author: "Murat Yücedağ".to_string(),
        page_count: 472,
    };
    let book3 = Book {
        title: "Arduino".to_string(),
        author: "Coşkun Taşdemir".to_string(),
        page_count: 280,
    };
    let magazine1 = Magazine {
        title: "Rust World".to_string(),
        issue: 42,
        topic: "Systems Programming".to_string(),
    };
    let magazine2 = Magazine {
        title: "Bilim Çocuk".to_string(),
        issue: 123,
        topic: "Science".to_string(),
    };

    let publications = vec![
        Publication::Book(book1),
        Publication::Book(book2),
        Publication::Book(book3),
        Publication::Magazine(magazine1),
        Publication::Magazine(magazine2),
    ];

    print_publications(publications);
}
