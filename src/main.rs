//use std::env;
pub mod regex_1;
pub mod lib_1;
pub mod import;
pub mod check_1;

fn main() {
    println!("Hello, world!");
    //проверка файлов и папок
    let _ = check_1::check_file_exists_1();
    //получение имен файлов
    let tuple = import::read_catalogs();
    //книги
    let books_path_vec=tuple.0;
    //словари
    let dictionary_path_vec: Vec<String>=tuple.1;

    for i in 0..books_path_vec.len() {
        println!("имя книги: {}",books_path_vec[i].name);
        for j in 0..books_path_vec[i].content.len() {
            println!("{}",&books_path_vec[i].content[j]);
        }
        println!("");
        println!("");
    }
}
