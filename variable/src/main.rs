fn main() {
    let mut x = 5 ;
    println!("The value of x is : {x}");
    x =6 ;
    println!("The value of x is : {x}");
    const PI: f32 = 3.14;
    println!("The value of pi is {PI}");

    let shadow_var = 10 ;
    let shadow_var = shadow_var + 5;

    {
        let shadow_var = shadow_var * 2;
        println!{"value of shadow_var in inner scope {shadow_var}"};
    }
    println!("The value of shadow_var is {shadow_var}");
}
