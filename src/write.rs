//use std::fs::read_to_string;
use std::fs::File;
use std::io::{//BufRead, BufReader, 
    Error, Write};
    use crate::lib_1;
    use encoding_rs::{WINDOWS_1251, DecoderResult};

pub fn write_book (book_struct:&Vec<lib_1::Books>) -> Result<(), Error> 
{
    for i in 0..book_struct.len() {
        //путь до вывода
        //let path = format!("./end/{}.{}",i,book_struct[i].format);
        let path = format!("./end/{}.{}",i,book_struct[i].format);
        //указание на вывод
    let mut output = File::create(path)?;
    //вывод книги
    for line in book_struct[i].content.iter() {
        //если это rtf
        if book_struct[i].format.contains("rtf") {
        let (_windows_1251_bytes, _, _) = WINDOWS_1251.encode(&line);
        // Преобразование UTF-8 → Windows-1251
        let windows1251_bytes = utf8_to_windows1251(&line);
        let (_s, _, had_errors) = WINDOWS_1251.decode(&windows1251_bytes);
        if had_errors {
        println!("Были ошибки декодирования");
        }
    output.write_all(& windows1251_bytes)?;
} else {
        writeln!(output, "{}",line)?;
}
    }

    }

    Ok(())

}
//из utf8 в Windows 1251 для RTF
fn utf8_to_windows1251(utf8_str: &str) -> Vec<u8> {
    let (result, _, had_errors) = WINDOWS_1251.encode(utf8_str);
    if had_errors {
        // Обработка символов, которые не могут быть представлены в Windows-1251
        eprintln!("Некоторые символы не могут быть представлены в Windows-1251");
    }
    result.into_owned()
}