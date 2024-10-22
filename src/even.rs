fn main() {
    let ans = is_even( 11) ;
    println!("{}", ans) ;
    
}
// 'i' means signed data- can be negative, 'u' means unsigned data- can't be negative
fn is_even(num: i32) -> bool {  
    if num % 2 == 0 {
        return true ;
    } else {
        return false;
    }
}
