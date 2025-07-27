struct Square{
    width: i32,
    hieght: i32,
}
impl Square{
    fn area(&self)->i32{
        self.width*self.hieght

    }
    fn premeter(&self)->i32{
        2*(self.width+self.hieght)
    }
    fn change_width(&mut self,x :i32){
        self.width=x;

    }
}
fn main() {
    let mut  sq=Square{width:5,hieght:5};
    println!("area of the square is {}",sq.area());
    println!("premeter of the square is {}",sq.premeter());
    sq.change_width(3);
    println!("{}",sq.width)


}
// fn main(){

//     let r;
//     {
//           let x=5;
//           r=&x;
          


//     }
//      println!("r:{}",r)
   
  


// }
