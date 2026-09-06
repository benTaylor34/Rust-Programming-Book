
fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    {
        let r1 = &mut s1;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s1;//you can have multiple mut references so long as theyre in different scope
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {// s is a reference to a String
    s.len()
}// Here s goes out of scope but beause s does not have ownership the string is not dropped