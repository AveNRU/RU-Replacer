//use std::fs::read_to_string;
use std::fs::File;
use std::io::{//BufRead, BufReader, 
    Error, Write};
    use crate::lib_1;


pub fn write_book (book_struct:&Vec<lib_1::Books>) -> Result<(), Error> 
{
    for i in 0..book_struct.len() {
        //путь до вывода
        let path = format!("./end/{}.{}",i,book_struct[i].format);
        //указание на вывод
    let mut output = File::create(path)?;
    //вывод книги
    for line in book_struct[i].content.iter() {
        writeln!(output, "{}", line).unwrap();
    }

    }

    Ok(())

}