use crate::Traits::{Display, Summary};

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn traitBoundNotification<T: Summary>(item: &T) {
    println!("Breaking News! {}", item.summarize());
}

pub fn multipleTraitBoundNotification(item: &(impl Summary + Display)) {
    println!("Fixing News! {}", item.display());
}