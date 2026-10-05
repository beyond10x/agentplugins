//! An in-memory lending library. Mutations and reads share one synchronous store.
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub state: &'static str,
    pub borrower: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Refusal {
    UnknownBook,
    WrongState(&'static str),
}

#[derive(Default)]
pub struct Library {
    pub books: BTreeMap<String, Book>,
    pub members: BTreeMap<String, String>,
    revision: u64,
    next_id: u64,
}

impl Library {
    fn id(&mut self) -> String {
        self.next_id += 1;
        format!("00000000-0000-4000-8000-{:012x}", self.next_id)
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn add_book(&mut self, title: String, author: String) -> String {
        let id = self.id();
        self.books.insert(
            id.clone(),
            Book {
                id: id.clone(),
                title,
                author,
                state: "OnShelf",
                borrower: None,
            },
        );
        self.revision += 1;
        id
    }

    pub fn register_member(&mut self, name: String) -> String {
        let id = self.id();
        self.members.insert(id.clone(), name);
        self.revision += 1;
        id
    }

    pub fn borrow(&mut self, book_id: &str, member_id: &str) -> Result<(), Refusal> {
        let book = self.books.get_mut(book_id).ok_or(Refusal::UnknownBook)?;
        if book.state != "OnShelf" {
            return Err(Refusal::WrongState(book.state));
        }
        book.state = "OnLoan";
        book.borrower = Some(member_id.to_owned());
        self.revision += 1;
        Ok(())
    }

    pub fn return_book(&mut self, id: &str) -> Result<(), Refusal> {
        let book = self.books.get_mut(id).ok_or(Refusal::UnknownBook)?;
        if book.state != "OnLoan" {
            return Err(Refusal::WrongState(book.state));
        }
        book.state = "OnShelf";
        book.borrower = None;
        self.revision += 1;
        Ok(())
    }

    pub fn withdraw(&mut self, id: &str) -> Result<(), Refusal> {
        let book = self.books.get_mut(id).ok_or(Refusal::UnknownBook)?;
        if book.state != "OnShelf" {
            return Err(Refusal::WrongState(book.state));
        }
        book.state = "Withdrawn";
        self.revision += 1;
        Ok(())
    }
}
