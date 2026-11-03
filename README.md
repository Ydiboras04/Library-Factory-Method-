# Library — Factory Method Pattern in Rust

A small library catalogue that demonstrates the **Factory Method** design pattern in Rust.

## What is the Factory Method pattern?

Factory Method is a creational design pattern. It **defines an interface for creating an object, but lets each implementation decide which concrete type to create**.

Without it, code that needs objects usually builds them directly:

```rust
let item = Book::new("Dune"); // the caller is tied to Book
```

Every place that does this depends on a concrete type. Adding a new type means finding and changing those places, usually by growing a `match` or an `if/else` chain.

With Factory Method, the caller asks for "an item" and gets back an abstract type. The choice of *which* item is pushed into a dedicated method that each implementation overrides.

## How it works

The pattern has four roles:

| Role | Responsibility |
|---|---|
| **Product** | The common interface of every object the factory can create. |
| **Concrete Product** | A specific implementation of the Product. |
| **Creator** | Declares the **factory method**, which returns a Product. It often also has shared logic that *uses* the factory method. |
| **Concrete Creator** | Overrides the factory method to return a specific Concrete Product. |

```
        Creator                                Product
 ┌───────────────────────┐   creates   ┌──────────────────┐
 │ create_item() -> P    │ ──────────► │ title()          │
 │ add_to_catalog()      │             │ kind()           │
 │   uses create_item()  │             │ loan_period_days │
 └───────────▲───────────┘             └────────▲─────────┘
             │ implements                       │ implements
 ┌───────────┴───────────┐             ┌────────┴─────────┐
 │ BookSection           │ ──────────► │ Book             │
 │ MagazineSection       │ ──────────► │ Magazine         │
 │ DvdSection            │ ──────────► │ Dvd              │
 │ AudioBookSection      │ ──────────► │ AudioBook        │
 └───────────────────────┘             └──────────────────┘
```

The key idea is that the Creator's shared logic is written **once**, against the abstract Product. It calls the factory method and never knows which concrete type comes back.

### In Rust

Rust has no class inheritance, so the pattern maps onto traits:

- Product → a **trait** (`LibraryItem`)
- Creator → a **trait** with a required factory method and **default methods** for the shared logic (`Section`)
- The abstract return type → a **trait object**, `Box<dyn LibraryItem>`

## Our code

```
src/
├── items.rs     # Product trait + concrete products
├── sections.rs  # Creator trait + concrete creators
└── main.rs      # Client code
```

### Product: `items.rs`

```rust
pub trait LibraryItem {
    fn title(&self) -> &str;
    fn kind(&self) -> &'static str;
    fn loan_period_days(&self) -> u32;
}
```

Every library item can report its title, its kind and how long it can be borrowed. The concrete products implement this trait:

| Concrete Product | `kind()` | `loan_period_days()` |
|---|---|---|
| `Book` | `"Book"` | 21 |
| `Magazine` | `"Magazine"` | 7 |
| `Dvd` | `"DVD"` | 3 |
| `AudioBook` | `"Audio Book"` | 14 |

The `title` fields are private, so the rest of the program can only reach an item through the `LibraryItem` trait.

### Creator: `sections.rs`

```rust
pub trait Section {
    // The factory method: each section decides what concrete item to create.
    fn create_item(&self, title: &str) -> Box<dyn LibraryItem>;
    fn name(&self) -> &'static str;

    // Shared logic, written once. It relies on create_item().
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
```

- `create_item` is **the factory method**. It returns `Box<dyn LibraryItem>`, meaning "some library item" without saying which one.
- `add_to_catalog` is the Creator's **shared logic**. It is a default method, so every section gets it for free. It never names `Book`, `Magazine` or any other concrete type.

### Concrete Creators

Each section implements only the factory method (plus its display name):

```rust
pub struct BookSection;

impl Section for BookSection {
    fn create_item(&self, title: &str) -> Box<dyn LibraryItem> {
        Box::new(Book::new(title))
    }

    fn name(&self) -> &'static str {
        "Book Section"
    }
}
```

`MagazineSection`, `DvdSection` and `AudioBookSection` follow the same shape, each returning its own product.

### Client: `main.rs`

```rust
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
```

The client only works with the `Section` trait. It doesn't import the `items` module and never names a concrete item type.

## Running it

```sh
cargo run
```

Output:

```
[Book Section] Catalogued Book "Dune" (loan: 21 days)
[Magazine Section] Catalogued Magazine "National Geographic" (loan: 7 days)
[DVD Section] Catalogued DVD "Inception" (loan: 3 days)
[Audio Book Section] Catalogued Audio Book "Project Hail Mary" (loan: 14 days)
```

The same `add_to_catalog` code produced all four lines. The factory method of each section decided which item was built.

## Why it's worth it

`AudioBook` was added after the first three types. Supporting it took:

1. a new `AudioBook` struct implementing `LibraryItem`,
2. a new `AudioBookSection` implementing `Section`,
3. one new entry in `main`'s list of sections.

The `LibraryItem` trait, `add_to_catalog`, the loop in `main` and the existing sections were **not modified**. This is the **Open/Closed Principle**: the code is open to extension but closed to modification.

Other benefits:

- **Loose coupling:** the shared logic and the client depend on traits, not concrete types.
- **Single place for creation:** each concrete type is built in exactly one factory method.
- **Easy to test:** a test can supply its own `Section` that returns a fake item.

The trade-off is more types: every new product needs a matching creator. For a handful of fixed types, a plain constructor is simpler. The pattern pays off when new kinds of objects are expected over time.
