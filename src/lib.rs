//! # Online Business
//! This is a rust library for online store

// ---------- Modules -----------
// Re-exporting without having to declare the whole module public
pub use customer::Customer;
pub use order::Order;
pub use product::{Category, Product};

mod product {
    // Bringing items into scope
    pub use category::Category;
    /// Struct for storing product related information
    #[derive(PartialEq, Debug)]

    // Making the struct public doesn't make it's fields public
    pub struct Product {
        id: u64,
        pub name: String,
        price: f64,
        category: Category,
    }

    impl Product {
        /// # Example
        /// ```
        /// use project_1234::Category;
        /// use project_1234::Product;
        /// let some_product = Product::new(1, String::from("Laptop"), 799.99, Category::Electronics);
        /// assert_eq!(some_product.name, String::from("Laptop"));
        /// ```
        pub fn new(id: u64, name: String, price: f64, category: Category) -> Product {
            Product {
                id,
                name,
                price,
                category,
            }
        }
    }

    // Unlike structs making enums public will make it's values public as well
    mod category {
        /// Enum for representing product category
        #[derive(PartialEq, Debug)]
        pub enum Category {
            Electronics,
            Clothing,
            Books,
        }
    }

    impl Product {
        fn calculate_tax(&self) -> f64 {
            self.price * 0.1
        }

        pub fn product_price(&self) -> f64 {
            self.price * self.calculate_tax()
        }
    }
}

mod customer {
    pub struct Customer {
        id: u64,
        name: String,
        email: String,
    }

    impl Customer {
        pub fn new(id: u64, name: String, email: String) -> Customer {
            Customer { id, name, email }
        }
    }
}

mod order {
    use crate::customer::Customer;
    use crate::product::Product;

    pub struct Order {
        id: u64,
        product: Product,
        customer: Customer,
        quantity: u32,
    }

    impl Order {
        pub fn new(id: u64, product: Product, customer: Customer, quantity: u32) -> Order {
            Order {
                id,
                product,
                customer,
                quantity,
            }
        }

        fn calculate_discount(&self) -> f64 {
            if self.quantity > 5 {
                0.1
            } else {
                0.0
            }
        }

        pub fn total_bill(&self) -> f64 {
            let discount = self.calculate_discount();
            let total_before_discount = self.product.product_price() * self.quantity as f64;
            total_before_discount - (total_before_discount * discount)
        }
    }
}
