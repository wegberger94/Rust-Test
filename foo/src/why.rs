fn erste_rechnung(x:u32)  -> u32{
       let mut y = x;
       if (x > 4)
       {
           y = y - 4;
       }
    y
}

fn main() {
   let mut zahl = String::new();
   let x;

   println!("Geben Sie eine Zahl ein: ");

   std::io::stdin().read_line(&mut zahl)
      .expect("Failed to read line");

   println!("{}",  zahl);

   let zahl2: u32 = zahl.trim().parse()
       .expect("Please type a number!");

   x = erste_rechnung(zahl2);


   println!("{}",  x);
}
