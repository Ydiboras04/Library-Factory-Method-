mod items;
mod sections;

use sections::{BookSection, DvdSection, MagazineSection, AudioBookSection, Section};

fn main() {
    //The client holds Creators only through the trait.
    let sections: Vec<Box<dyn Section>> = vec![
        Box::new(BookSection),
        Box::new(MagazineSection),
        Box::new(DvdSection),
        Box::new(AudioBookSection),
    ];

    let titles = ["Dune", "National Geographic", "Inception", "Project Hail Mary"];

    for (section, title) in sections.iter().zip(titles) {
        section.add_to_catalog(title);
    }
}
