use std::env;

use lib_1::Dictionary;
pub mod check_1;
pub mod dictionary_0;
pub mod import;
pub mod import_xls;
pub mod lib_1;
pub mod regex_1;
pub mod test_0;
pub mod write;

fn main() {
    unsafe { env::set_var("RUST_BACKTRACE", "full") };
    unsafe { env::set_var("RUSTFLAGS", "-Awarnings") };
    unsafe { env::set_var("RUSTFLAGS", "-A dead_code") };
    println!("Исполнение от 15 мая 2025 года");

    //проверка файлов и папок
    let _ = check_1::check_file_exists_1();
    //получение имен файлов
    let tuple: (Vec<lib_1::Books>, Vec<String>) = import::read_catalogs();
    //книги
    let books_struct_original: Vec<lib_1::Books> = tuple.0;

    //словари
    let dictionary_path_vec: Vec<String> = tuple.1;
    //словарь со словами в виде заглвных букв и маленьких
    let dictionary_lib: Vec<Dictionary> = import_xls::import_dictionary(&dictionary_path_vec);
    //книги изменённые
   /*  let books_struct_changed: Vec<lib_1::Books> =
        dictionary_0::change_words_in_books(&dictionary_lib, &books_struct_original);
    let _ = write::write_book(&books_struct_changed);
    */
}
