fn main() {
    print_phrase("hello world    ");
    
    println!("{}",add(12, 1));
    println!("{}",greater_common_denominatior(20, 3));
    println!("{}",is_greater_than_5(5));

}
fn print_phrase(phrase: &str) {
    println!("{}", phrase);
}
fn add(x:i8,y:i8)->i8{
   x+y
}

fn greater_common_denominatior(mut x:u64,mut y:u64)->u64{

    while x!=0{
        if x<y{
            let c =x;
            x=y;
            y=c;


        }
        x=x%y
    }
    y

}
fn is_greater_than_5(x:i8)->bool{
    if x>5{
        true
    }else {
        false
    }
}