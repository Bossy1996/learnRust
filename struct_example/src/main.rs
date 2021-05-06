fn main() {
   let width1 = 30;
   let heigth1 = 50;

   println!(
       "The area of the rectangle is {} square pixels.",
       area(width1, heigth1)
   );
}

fn area(width1: u32, heigth1: u32) -> u32 {
    width1 * heigth1
}
