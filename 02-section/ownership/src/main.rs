fn main() {
    // let var =1;//created on the stack
    // let mut s="hello".to_string();//created on the heap
    // s.push_str(" world");
    // println!("{} , {}",s,var)


    // let x=vec!["tyler".to_string()];
    // let y=x.clone();
    // let _z=y.clone();
    // println!("x is {:?}",x);
    //  println!("y is {:?}",y);

    // let x=vec!["tyler".to_string()];
    // let y=x;
    // let z=y;
   
    // println!("z is {:?}",z);

    // let x=1;

    // let y = x;
    // println!("x= {}, y={}",x,y)

    let mut s = String::from("hello");
    change_string(&mut s);
    println!("s is {:?}",s)

}

fn change_string(some_string : &mut String){
some_string.push_str(" world");
}
//var and s are droppedz is
