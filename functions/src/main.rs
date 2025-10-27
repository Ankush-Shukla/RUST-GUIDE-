fn main() {
    sum(5 , 7);
    println!("Sum function called");
    
    let y = {
        let x = 3 ;
        x +1 
    };

    println!("The value of y is : {y}");
    let x = pi();
    println!("the value of PI is {x} ");
    println!("incrementing 5 by 1 : {}",increment(5));


}

fn sum(x:i32,y:i32){
   println!("sum of {x}+{y} : {} ",x+y);

}

fn pi()-> i32 {
    3
}

fn increment(x: i32) -> i32{
    x+1
}
