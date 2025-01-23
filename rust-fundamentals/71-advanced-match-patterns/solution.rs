#[derive(Clone, Debug)]
pub enum BookItem {
    Book { pages: i32, discount: Option<i32> },
    EBook(String, (i32, i32)),
    Collection(Vec<BookItem>),
    OutOfPrint,
}

impl BookItem {
    pub fn check_validity(&self) -> bool {
        match self {
            BookItem::Book { pages, discount } => {
                *pages > 0 && if let Some(disc_val) = *discount {
                    disc_val >= 0 && disc_val <= 50
                } else {
                    true
                }
            },
            BookItem::EBook(title, (_, value)) => { !title.is_empty() && *value > 0 },
            BookItem::Collection(list) => {
                list.len() > 0 && !list.iter().any(|x| {!x.check_validity()})
            },
            BookItem::OutOfPrint => { false },
        }
    }
}

// Example usage
pub fn main() {
    let book_a = BookItem::Book {
        pages: 42,
        discount: Some(100),
    };
    let ebook_b = BookItem::EBook("hello".to_string(), (1, 2));
    let collection_c = BookItem::Collection(vec![book_a.clone(), BookItem::OutOfPrint]);

    assert!(
        !book_a.check_validity(),
        "Book with discount > 50 should be invalid"
    );
    assert!(
        ebook_b.check_validity(),
        "EBook with valid title and tuple should be valid"
    );
    assert!(
        !collection_c.check_validity(),
        "Collection containing invalid items should be invalid"
    );
    assert!(
        !BookItem::OutOfPrint.check_validity(),
        "OutOfPrint should always be invalid"
    );
}

