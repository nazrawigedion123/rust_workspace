fn main() {
    let name= String::from("nazri ናዝራዊ");
    let course = "Rust".to_string();
    println!("name :{} , course: {}",name,course);

    let new_name=name.replace("nazri ናዝራዊ", "Nazrawi gedion");
    println!("name :{} ",new_name);
}
