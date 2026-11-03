use crate::items::{Book, LibraryItem, Magazine, Dvd, AudioBook};

pub trait Section {
    //the factory method: each section decide what concrete item to create
    fn create_item(&self, title: &str) -> Box<dyn LibraryItem>;
    fn name(&self) -> &'static str;

    fn add_to_catalog(&self, title: &str) -> Box<dyn LibraryItem> {
        let item = self.create_item(title);
        println!(
            "[{}] Catalogued {} \"{}\" (loan: {} days)",
            self.name(),
            item.kind(),
            item.title(),
            item.loan_period_days()
        );
        item
    }
}

pub struct BookSection;

impl Section for BookSection {
    fn create_item(&self, title: &str) -> Box<dyn LibraryItem> {
        Box::new(Book::new(title))
    }

    fn name(&self) -> &'static str {
        "Book Section"
    }
}

pub struct MagazineSection;

impl Section for MagazineSection {
    fn create_item(&self, title: &str) -> Box<dyn LibraryItem> {
        Box::new(Magazine::new(title))
    }

    fn name(&self) -> &'static str {
        "Magazine Section"
    }
}

pub struct DvdSection;

impl Section for DvdSection {
    fn create_item(&self, title: &str) -> Box<dyn LibraryItem> {
        Box::new(Dvd::new(title))
    }

    fn name(&self) -> &'static str {
        "DVD Section"
    }
}

pub struct AudioBookSection;

impl Section for AudioBookSection {
    fn create_item(&self, title: &str) -> Box<dyn LibraryItem> {
        Box::new(AudioBook::new(title))
    }

    fn name(&self) -> &'static str {
        "Audio Book Section"
    }
}