mod math
{
    pub mod basic
    {
        pub fn sum(first_number:i32,second_number:i32)->i32
        {
            return first_number + second_number;
        }
    }
}

fn main() {
    
   println!("Sum of two number using module in main.rs file {} " , math::basic::sum(50, 50));

}
