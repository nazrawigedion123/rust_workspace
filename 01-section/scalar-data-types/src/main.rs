
/*
Scalar types represent a single value 
4 types of scalar data-types 
int
float
char
boolean

*/
fn main() {
    //integers
    println!("__________________________________________________________
    integers 
__________________________________________________________");

// operators * / - + %
    let x:i8 = 50;// i8 being an 8 bit integer 

    println!(" x is {}", x);

    let y : u8 =10;
    println!("y is {}",y);

    let decimal = 02_55;
    let hex= 0xff;
    let octal=0o377;
    let binary=0b1111_1111;
    println!("decimal is {}",decimal);
    println!("hex is {}",hex);
    println!("octal is {}",octal);
    println!("binary is {}",binary);
    let byte = b'A';
    println!("byte is {}",byte);

    //floats
    println!("__________________________________________________________
    floats
    __________________________________________________________");

    let f = 2.0; //f64 is the default , because in modern cpus it the same speed as f32
    let v: f32=1.0;

    println!(" f is {}",f);
    println!("f32 is {}",v);

    //boolean
    println!("__________________________________________________________
    boolean
     __________________________________________________________");

     let t =true;
     let f :bool =false;

     println!("t is {}",t);
     println!("f is {}",f);

         //Char
    println!("__________________________________________________________
    Char
     __________________________________________________________");

     let b = 'b';
     let c : char='c';
     println!("b is {}",b);
     println!("c is {}",c);





}
