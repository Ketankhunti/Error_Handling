use std::io::{ErrorKind, Read};
use std::fs::{self,File};
use std::io;

fn main() {
   let f = File::open("hello.txt");
    let file = match f {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) =>file,
                Err(_err) => panic!("Couldn't create file : {}",_err)
            },
            other_error => panic!("Couldn't open file : {:#?}",other_error)
        }
    };

   let  k = read_username();
    println!("{:#?}",k);
}

fn read_username() -> std::io::Result<String> {
   /* let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)*/
   match  fs::read_to_string("hello.txt"){
       Ok(string) => Ok(string),
       Err(error) => Err(error)
   }
}
