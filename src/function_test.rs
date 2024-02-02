pub fn test_function() {
    let a = "HEia";
    let mut s = String::from("hello");
    s.push_str(", world!");
    s.push_str((" "));
    println!("{}", s);
    println!("{}", a);
}
