pub trait LibraryItem {
    fn title(&self) -> &str;
    fn kind(&self) -> &'static str;
    fn loan_period_days(&self) -> u32;
}

pub struct Book {
    title: String,
}

pub struct Magazine {
    title: String,
}

pub struct Dvd {
    title: String,
}

pub struct AudioBook {
    title: String,
}

impl Book {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }
}

impl Magazine {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }
}

impl Dvd {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }
}

impl AudioBook {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }
}

impl LibraryItem for Book {
    fn title(&self) -> &str {
        &self.title
    }

    fn kind(&self) -> &'static str {
        "Book"
    }

    fn loan_period_days(&self) -> u32 {
        21
    }
}

impl LibraryItem for Magazine {
    fn title(&self) -> &str {
        &self.title
    }

    fn kind(&self) -> &'static str {
        "Magazine"
    }

    fn loan_period_days(&self) -> u32 {
        7
    }
}

impl LibraryItem for Dvd {
    fn title(&self) -> &str {
        &self.title
    }

    fn kind(&self) -> &'static str {
        "DVD"
    }

    fn loan_period_days(&self) -> u32 {
        3
    }
}

impl LibraryItem for AudioBook {
    fn title(&self) -> &str {
        &self.title
    }

    fn kind(&self) -> &'static str {
        "Audio Book"
    }

    fn loan_period_days(&self) -> u32 {
        14
    }
}