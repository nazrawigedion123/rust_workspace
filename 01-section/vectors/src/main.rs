fn main() {
    /*
    Vectors are resizable array of elements allocated on the heap
    
     */
    let mut nums = vec![1,2,3];
    nums.push(4);
    println!("{:?}",nums);
    nums.pop();
    println!("{:?}",nums);

    let mut vec= Vec::new();
    vec.push(1);
    vec.push(6);
    println!("{:?}",vec);
    vec.reverse();
    println!("{:?}",vec);

    let vect = Vec::<i32>::with_capacity(2);
    println!("{}",vect.capacity());

    let v: Vec<i32>=(0..5).collect();
    println!("{:?}",v);

    // splice points to a vector or part of a vector

    let sv:&[i32]=&v;
    println!("{:?}",sv);

    let jv :&[i32]=&v[2..];
    println!("{:?}",jv);
}
