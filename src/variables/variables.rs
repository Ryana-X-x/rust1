// ********** Numbers ************
// fn main() {
//     let x: i32 = -5 ;
//     let y: u32 = 10 ;
//     let z: f32 = 1000.90 ;

//     println!("x: {}, y: {}, z: {}", x, y, z) ;
// }

// fn main(){
//     let mut x: i8 = 10 ;

//     for i in 0..1000 {
//         x = x + 100 ;   // > 127
//     }

//     println!("x = {}",x ) ;      // attempt to add with overflow
// }

// ********* Boolean ************

// fn main(){
//     let is_male = false ;
//     let is_above = true ; 

//     if is_male {
//         println!("You are a male") ;
//     }
//     else{
//         println!("You are not a male") ;
//     }

//     if is_male && is_above {
//         println!("You are a legal male!") ;
//     }
// }


// ******* Strings ************
// strings -> dont have a fixed type
fn main() {
    let greeting = String::from("Hello world") ;
    println!("{}", greeting) ;

    //println!("{}", greeting.chars().nth(0)) ;   // Option<char>
}

    