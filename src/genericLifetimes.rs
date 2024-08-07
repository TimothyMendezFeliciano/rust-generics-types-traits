fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // In my case both parameters have a lifetime declaration. Which would allow the function to work.
    if x.len() > y.len() {
        x
    } else {
        y
    }
}