fn main() {
    let s = String::from("Test");
    heap_example(s);
}
fn heap_example(input: String)  {
   let mystr = input;
    let _anotherstr = mystr.clone();
    println!("{}", mystr);
}