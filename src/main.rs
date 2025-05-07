use std::env;
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
    println!("Hello, world!");
    //проверка файлов и папок
    let _ = check_1::check_file_exists_1();
    //получение имен файлов
    let tuple = import::read_catalogs();
    //книги
    let books_struct_original=tuple.0;
    //словари
    let dictionary_path_vec: Vec<String>=tuple.1;
    //println!("{:?}",&dictionary_path_vec);

    let _ =write::write_book(&books_struct_original);
}
