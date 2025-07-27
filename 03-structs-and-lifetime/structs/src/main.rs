//Named structs
// #[derive(Debug)] 
// struct User{
//     active:bool,
//     username:String,
//     signin_count:u32,

// }

// fn main() {
//     let user1=build_user("nazrawi".to_string());
//     println!("{:#?}",user1);
// }
// fn build_user(username: String)->User{
//    User{
//         username,
//         active:true,
//         signin_count:0,
//     }
    
// }

//tuple like structs
#[derive(Debug)]
struct Coordinates(i32,i32,i32);

fn main(){
    let c1=Coordinates(1,2,3);
    println!("{:?}",c1)
}