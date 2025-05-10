fn main() {
    /*
    rust has two compound data types them being 
    tuples and arrays

     */


    /*
    tuples is a general way of grouping of variety of types in to one compound type
    tuples have a fixed length so they cant grow or shrink once they have been decared 
     */

    let tup= (500,"hi",true);
    let (x,y,z)= tup;
    println!("Tuples");
    println!("{}",x);
    println!("{}",y);
    println!("{}",z);

    /*
    array are similar to tuples in that they have fixed sizes 
    but arrays can only have the same types of elements in a grouping
    
     */
    let array=[1,2,3];

    println!("\n\narrays");

    println!("{}",array[0]);

    let mut array2:[i32;3]=[4,5,6];
    array2[1]=1;
    
    println!("{}",array2[0]);

    
}
