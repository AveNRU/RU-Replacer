use std::env;

use lib_1::Dictionary;
pub mod regex_1;
pub mod lib_1;
pub mod write;
pub mod import;
pub mod import_xls;
pub mod check_1;

fn main()  {
    
    unsafe { env::set_var("RUST_BACKTRACE", "full") };
    unsafe { env::set_var("RUSTFLAGS", "-Awarnings") };
    unsafe { env::set_var("RUSTFLAGS", "-A dead_code") };
    
    //проверка файлов и папок
    let _ = check_1::check_file_exists_1();
    //получение имен файлов
    let tuple = import::read_catalogs();
    //книги
    let books_struct_original=tuple.0;
    //словари
    let dictionary_path_vec: Vec<String>=tuple.1;
    let dictionary_lib=import_xls::import_dictionary(&dictionary_path_vec);
    //вывод слов
    for i in 0..dictionary_lib.len() {
        for j in 0..dictionary_lib[i].single.len() {
           // println!("j: {j}");
            for k in 0..dictionary_lib[i].single[j].len () {
           // println!("слово: {}",&dictionary_lib[i].single[j][k]);
            }
        }
    }
    //println!("{:?}",&dictionary_path_vec);

    let _ =write::write_book(&books_struct_original);
}
