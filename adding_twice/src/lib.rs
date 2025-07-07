pub use adding::add_curry;

pub fn twice<T:Fn(i32) -> i32>(F:T) -> impl Fn(i32) -> i32{
    move |a| F(F(a))
}