use std::io::{//self, 
    BufRead, BufReader,// Error, 
    Read, //Write
};
use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;
//use std::path::Path;
use std::fs::{self, 
    //metadata, 
    //File
};

//use std::fs::read_to_string;
use crate::lib_1::{self, Dictionary};

use regex::Regex;


pub fn read_books() ->Vec<lib_1::Books> {
    let sa:Vec<lib_1::Books>=Vec::new();

    return sa
}

//Чтение файлов
//1 - книги, 2 - словари
pub fn read_catalogs() -> (Vec<lib_1::Books>, Vec<String>) {
    use std::fs::{self, 
        //metadata, 
        File
    };
    //основной путь
    let mut main_path:String=String::new();
    //получение значение корневого доступа к скрипту (где он лежит, как решила ОС)
    let _ = file_full_path_env(&mut main_path);
        let mut _book_struct :Vec<lib_1::Books>=Vec::new();
         //файлы книг
        let books_vec = fs::read_dir("./books/")
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        //словари
        let dictionary_vec = fs::read_dir("./dictionary/")
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        for i in 0..books_vec.len() {
            //открытие файла с библиотекой
        let result = File::open(&books_vec[i]);
        //проверка итога открытия
    match result {
        Ok(_) => (),
        Err(_) => panic!("Файл {} не существует!: ",&books_vec[i]),
    }
       // let _s1=books_vec[i].replace(".", ""));
        //let _name=_s1.replace();
        /*let mut _str_vec:Vec<String>= Vec::new();
        for line in read_to_string(&books_vec[i]).unwrap().lines() {
            //вложение строк файла .spd в вектор
            _str_vec.push(line.to_string())
        }*/
        let _str_vec=read_utf8(&books_vec[i]); //чтение файла в UTF-8
        //вложение
        let _time_struct = lib_1::Books {
            content: _str_vec,//содержимое книги
            path: books_vec[i].clone(),//путь полный
            name: books_vec[i].clone(),//имя книги
            format: "rtf".to_string(),
        };
        _book_struct.push(_time_struct);
        }
    return (_book_struct, dictionary_vec);
}


//получение пути до корня со скриптом в ОС
pub fn file_full_path_env(sivkov_path:&mut String) -> std::io::Result<()> {
    use std::env;
    let path = env::current_dir()?;
    //println!("The current directory is {}", path.display());
    *sivkov_path=path.into_os_string().into_string().unwrap();
    //println!("Итог пути: {}",&s);
    Ok(())
}




//чтение файла в UTF-8
pub fn read_utf8(rpt_path: &String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new(); //вектор строк - куда все помещается
    let file_path: &str = rpt_path; //путь до файла
    let buffer: Box<dyn BufRead> = read_file(file_path); //чтение файла
    for (index, result_vec_bytes) in buffer.split(b'\n').enumerate() {
        //перебор всех строк и переход на новые строки
        let line_number: usize = index + 1;
        let line_utf8: String = get_string_utf8(result_vec_bytes, line_number); //сохранение строки как UTF-8
        result.push(line_utf8.to_string()) //добавление строки в вектор
    }
    return result;
}
//чтение файла
fn read_file(file_path: &str) -> Box<dyn BufRead> {
    let file = match fs::File::open(file_path) {
        //попытка открытия файла
        Ok(f) => f,
        Err(why) => {
            //если ошибка
            println!("Ошибка при открытии файла: \"{file_path}\" по причине: \n{why:?}");
            system_pause();
            panic!("Ошибка при открытии файла: \"{file_path}\" по причине: \n{why:?}")
        }
    };
    Box::new(BufReader::new(file))
}
//получение строки в виде UTF-8
fn get_string_utf8(
    result_vec_bytes: Result<Vec<u8>, std::io::Error>, //вектор байт
    line_number: usize,                                //номер строки
) -> String {
    let vec_bytes: Vec<u8> = match result_vec_bytes {
        //попытка сопоставить вектор байт
        Ok(values) => values,
        Err(why) => {
            println!("Ошибка при чтении строки: |{line_number}| по причине: {why}");
            system_pause();
            panic!("Ошибка при чтении строки: |{line_number}| по причине: {why}")
        }
    };

    let line_utf8: String = match std::str::from_utf8(&vec_bytes) {
        Ok(str) => str.to_string(),
        Err(_) => {
            let mut data = DecodeReaderBytesBuilder::new()
                .encoding(Some(WINDOWS_1251))
                .build(vec_bytes.as_slice());

            let mut buffer = String::new();
            let _number_of_bytes = match data.read_to_string(&mut buffer) {
                Ok(num) => num,
                Err(why) => {
                    eprintln!("Сбой при чтении данных из файла в ОЗУ!");
                    eprintln!("Строка № {line_number}");
                    eprintln!("Используемая кодировка: WINDOWS_1251.");
                    eprintln!("Попробуйте другой вид кодировки!");
                    println!("Ошибка при преобразовании данных в UTF-8 по причине: {why}");
                    system_pause();
                    panic!("Ошибка при преобразовании данных в UTF-8 по причине: {why}")
                }
            };
            buffer
        }
    };
    // remove Window new line: "\r\n"
    line_utf8.trim_end_matches('\r').to_string()
}

//вывод паузы для windows - нажмите любую клавишу
pub fn system_pause() {
    use std::process::Command;
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

