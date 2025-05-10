fn main() {
    let a=1;
    if a>0{
        println!("{} is greater than 0",a );
    }else {
        println!("{} is less than 0",a)
    }



    let mut num=0;
    'counter :loop {
        println!("counter equals {}",num);
        let mut decrese = 5;
        loop {
            println!("decreasing : {}",decrese);
            if decrese==4{
                break;
            }
            if num==2{
                break 'counter;
            }
            decrese -=1
        }
        num+=1
        
    }

    let mut num=0;
    while num <5{
        println!("{}",num);
        num+=1;

    }

    let vec: Vec<i8>=(0..10).collect();
    for element in vec{
        println!("{}",element);
    }

    for num in (1..4).rev(){
        println!("{}",num);
    }
    println!("lift off")
}
