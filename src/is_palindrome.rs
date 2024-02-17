// First check if the number is negative i.e, less than 0 then it will not be pallindrome, so return False.
// If the number is greater then 0, then find the reverse of actual number.
// Comapre if reverse and actual number are equal. if equal return True, else False.
// Note : - Remember to hold the actual number in a variable , so that we can use it to compare because after finding Reverse number the input (here x) will be changed.
// Time Complexity :-O(n)

fn main() {
    println!("Hello, world!");
    let x = solution::reverse_num(121);
    println!("{:#?}",x);
    let y = solution::palindrome(121);
    println!("{:#?}",y);
}
pub struct solution {
    pub num : i32,
}

impl solution{
    pub fn reverse_num(x: i32) -> i32 {
        let mut reversed_num = 0;
        let mut num = x;
        while num != 0 {
            let last_digit = num % 10;
            num = num / 10;

            reversed_num = reversed_num * 10 + last_digit;
        }
        /* `i32` value */
        reversed_num
    }
    pub fn palindrome(x: i32) -> bool{
        let mut reversed_num = solution::reverse_num(x);
        let mut num = x;
        if x < 0{
            return false;
        }
        num == reversed_num


    }
}