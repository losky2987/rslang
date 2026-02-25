fn main() {
   let mut numbers = [1, 2, 3, 4, 5];

   // Take a slice of the array
   let mut_slice = &mut numbers[1..3]; // Slice from [1,..3(

   mut_slice[4] = 1;
}