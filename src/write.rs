//use std::fs::read_to_string;
use crate::lib_1;
use encoding_rs::{
    WINDOWS_1251,
    //    DecoderResult
};
use rust_xlsxwriter::*;
use std::fs::File;
use std::io::{
    //BufRead, BufReader,
    Error,
    Write,
};

pub fn write_book(book_struct: &Vec<lib_1::Books>) -> Result<(), Error> {
    for i in 0..book_struct.len() {
        //путь до вывода
        //let path = format!("./end/{}.{}",i,book_struct[i].format);
        let path = format!("./end/{}.{}", i, book_struct[i].format);
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
                output.write_all(&windows1251_bytes)?;
            } else {
                writeln!(output, "{}", line)?;
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

//вывод словарей
pub fn excel_dictionary_write(
    _dictionary: &Vec<lib_1::Dictionary>,
    //mode: String,           //Стопка из файла .xlsx взята или самостоятельно высчитана
    //path_name_spd: &String, //имя .spd файла
) -> Result<(), XlsxError> {
    // Create a new Excel file object.
    let mut workbook = Workbook::new();
    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();
    worksheet.write(0, 0, "Изначальные слова")?;
    worksheet.write(0, 1, "Regex")?;
    worksheet.write(0, 2, "Замена")?;
    worksheet.write(0, 3, "Количество случаев")?;
    worksheet.write(0, 4, "Строка")?;
    //worksheet.write(0, 5, "Ток потребления")?;
    //worksheet.write(0, 6, "Цепь земли (по умолчанию)")?;
    let mut _row_point: u32 = u32::try_from(1).unwrap().into();
    //let column_point: u16 = u16::try_from(i + 1).unwrap().into();
    //перебор всех словарей
    for i in 0.._dictionary.len() {
        //если все слова равны
        if _dictionary[i].single.len() == _dictionary[i].re_single.len()
            && _dictionary[i].single.len() == _dictionary[i].change_single.len()
        {
            println!("длина словаря (простого) : {}", _dictionary[i].single.len());
        }
        //если длина словаря не равна
        else {
            println!("длина слов простых: {}", _dictionary[i].single.len());
            println!("длина слов re_простых: {}", _dictionary[i].re_single.len());
            println!(
                "длина слов замен (простых): {}",
                _dictionary[i].change_single.len()
            );
        }
        //перебор одиночных слов
        for j in 0.._dictionary[i].single.len() {
            worksheet.write(_row_point, 0, _dictionary[i].single[j].clone())?;
            _row_point += 1;
            //println!("{}",&_dictionary[i].single[j]);
        }
        //обнуление указателя
        let mut _row_point: u32 = u32::try_from(1).unwrap().into();
        //вывод regex
        for j in 0.._dictionary[i].re_single.len() {
            worksheet.write(_row_point, 1, _dictionary[i].re_single[j].to_string())?;
            _row_point += 1;
        }
        //обнуление указателя
        let mut _row_point: u32 = u32::try_from(1).unwrap().into();
        //вывод regex
        for j in 0.._dictionary[i].change_single.len() {
            worksheet.write(_row_point, 2, _dictionary[i].change_single[j].to_string())?;
            _row_point += 1;
        }
    }
    //2-я страница с составными словами
    let mut binding = Worksheet::new();
    let complex = binding.set_name("Сложные слова")?;
    complex.write(0, 0, "Изначальные слова")?;
    complex.write(0, 1, "Regex")?;
    complex.write(0, 2, "Замена")?;
    complex.write(0, 3, "Количество случаев")?;
    complex.write(0, 4, "Строка")?;
    //complex.write(0, 5, "Ток потребления")?;
    //complex.write(0, 6, "Цепь земли (по умолчанию)")?;
    let mut _row_point: u32 = u32::try_from(1).unwrap().into();
    //let column_point: u16 = u16::try_from(i + 1).unwrap().into();
    //перебор всех словарей
    for i in 0.._dictionary.len() {
        if _dictionary[i].complex.len() == _dictionary[i].re_complex.len()
            && _dictionary[i].complex.len() == _dictionary[i].change_complex.len()
        {
            println!(
                "длина словаря (сложного) : {}",
                _dictionary[i].complex.len()
            );
        }
        //если длина словаря не равна
        else {
            println!("длина слов сложных: {}", _dictionary[i].complex.len());
            println!("длина слов re_сложных: {}", _dictionary[i].re_complex.len());
            println!(
                "длина слов замен (сложных): {}",
                _dictionary[i].change_complex.len()
            );
        }
        //перебор одиночных слов
        for j in 0.._dictionary[i].complex.len() {
            complex.write(_row_point, 0, _dictionary[i].complex[j].clone())?;
            _row_point += 1;
            //println!("{}",&_dictionary[i].complex[j]);
        }
        //обнуление указателя
        let mut _row_point: u32 = u32::try_from(1).unwrap().into();
        //вывод regex
        for j in 0.._dictionary[i].re_complex.len() {
            complex.write(_row_point, 1, _dictionary[i].re_complex[j].to_string())?;
            _row_point += 1;
        }
        //обнуление указателя
        let mut _row_point: u32 = u32::try_from(1).unwrap().into();
        //вывод regex
        for j in 0.._dictionary[i].change_complex.len() {
            complex.write(_row_point, 2, _dictionary[i].change_complex[j].to_string())?;
            _row_point += 1;
        }
    }

    //2-я страница с составными словами
    let mut binding2 = Worksheet::new();
    let everywhere = binding2.set_name("Вездесущие слова")?;
    everywhere.write(0, 0, "Изначальные слова")?;
    everywhere.write(0, 1, "Regex")?;
    everywhere.write(0, 2, "Замена")?;
    everywhere.write(0, 3, "Количество случаев")?;
    everywhere.write(0, 4, "Строка")?;
    //everywhere.write(0, 5, "Ток потребления")?;
    //everywhere.write(0, 6, "Цепь земли (по умолчанию)")?;
    let mut _row_point: u32 = u32::try_from(1).unwrap().into();
    //let column_point: u16 = u16::try_from(i + 1).unwrap().into();
    //перебор всех словарей
    for i in 0.._dictionary.len() {
        if _dictionary[i].everywhere.len() == _dictionary[i].re_everywhere.len()
            && _dictionary[i].everywhere.len() == _dictionary[i].change_everywhere.len()
        {
            println!(
                "длина словаря (сложного) : {}",
                _dictionary[i].everywhere.len()
            );
        }
        //если длина словаря не равна
        else {
            println!("длина слов сложных: {}", _dictionary[i].everywhere.len());
            println!("длина слов re_сложных: {}", _dictionary[i].re_everywhere.len());
            println!(
                "длина слов замен (сложных): {}",
                _dictionary[i].change_everywhere.len()
            );
        }
        //перебор одиночных слов
        for j in 0.._dictionary[i].everywhere.len() {
            everywhere.write(_row_point, 0, _dictionary[i].everywhere[j].clone())?;
            _row_point += 1;
            //println!("{}",&_dictionary[i].everywhere[j]);
        }
        //обнуление указателя
        let mut _row_point: u32 = u32::try_from(1).unwrap().into();
        //вывод regex
        for j in 0.._dictionary[i].re_everywhere.len() {
            everywhere.write(_row_point, 1, _dictionary[i].re_everywhere[j].to_string())?;
            _row_point += 1;
        }
        //обнуление указателя
        let mut _row_point: u32 = u32::try_from(1).unwrap().into();
        //вывод regex
        for j in 0.._dictionary[i].change_everywhere.len() {
            everywhere.write(_row_point, 2, _dictionary[i].change_everywhere[j].to_string())?;
            _row_point += 1;
        }
    }

    //путь сохранения
    let _path: String = format!("./end/2.xlsx",);
    complex.autofit();
    everywhere.autofit();
    worksheet.autofit();
    workbook.push_worksheet(binding);
    workbook.push_worksheet(binding2);
    workbook.save(_path)?;
    Ok(())
}
