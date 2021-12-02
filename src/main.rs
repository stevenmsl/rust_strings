fn main() {
    update_a_string();
    use_the_plus_operator();
    use_the_format_macro();
    iterate_over_string();
}

fn update_a_string() {
    // we are updating s1; it needs to be immutable
    let mut s1 = String::from("foo");
    let s2 = "bar"; //s2's type is &str and it's immutable
    s1.push_str(s2); //push_str will not take ownership of s2
    println!("s2 is {}", s2);
}

fn use_the_plus_operator() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    /* + operator
       - use add(self, s: &str) -> String {...}
         - add will take ownership of s1,
           append a copy of the contents of s2,
           and return the ownership of the result
         - that's why you can't use s1 anymore
         - you still can use s2

       - the type of &s2 is &String
         - Rust can coerce &String into &str
         - here Rust uses deref coercion to turn
           &s2 into &s2[..]
    */
    let s3 = s1 + &s2; //s1 is moved; you can't use it anymore

    println!("s3 is {}", s3);
}

fn use_the_format_macro() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("s is {}", s);
    println!("s1 is {}", s1); //you still can use s1
}

fn iterate_over_string() {
    /* नमस्ते
       - it is stored as a vector of u8; it has 18 bytes
       - it has 4 grapheme clusters (letters)
       - it has 6 scalar values (chars)
         ['न', 'म', 'स', '्', 'त', 'े']
         - the fourth and sixth are diacritics
         - combine third and fourth you get
           the third grapheme cluster (letter)
    */

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
