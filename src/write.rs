//use std::fs::read_to_string;
use rust_xlsxwriter::*;
use std::fs::File;
use std::io::{//BufRead, BufReader, 
    Error, Write};
    use crate::lib_1;
    use encoding_rs::{WINDOWS_1251, 
    //    DecoderResult
    };

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
    for i in 0.._dictionary.len(){
        println!("длина слов простых: {}",_dictionary[i].single.len());
        println!("длина слов re_простых: {}",_dictionary[i].re_single.len());
        println!("длина слов замен: {}",_dictionary[i].change_single.len());
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
   

    /* 
    for i in 0..x_connector.len() {
        //если количество цепей больше 1
        if x_connector[i].nets.len() > 0 {
            // println!("разъём: {:?}",x_connector[i]);
            _row_point += 1;
            worksheet.write(_row_point, 0, x_connector[i].refdes.clone())?;
            for j in 0..x_connector[i].nets.len() {
                worksheet.write(_row_point, 1, x_connector[i].nets[j].name.clone())?;
                // let volt=x_connector[i].nets[j].voltage.clone();
                //worksheet.write(_row_point, 2, x_connector[i].nets[j].voltage.clone())?;
                worksheet.write(
                    _row_point,
                    2,
                    x_connector[i].nets[j].voltage.to_string().clone(),
                )?;
                //вывод цепей земли разъёма
                if mode == ".spd" {
                    //все цепи
                    worksheet.write_row(_row_point, 6, x_connector[i].ground_nets.iter())?;
                    /*for n in 0..x_connector[i].ground_nets.len() {
                    worksheet.write(_row_point, (6+n) as u16,
                     x_connector[i].ground_nets[n].clone())?;
                    }
                    */
                }
                if mode == "xlsx" {
                    //если источник
                    if x_connector[i].nets[j].source {
                        //вывод знака что источник в 4 столбец
                        worksheet.write(_row_point, 3, "+")?;
                    }
                    //если потребитель
                    else if x_connector[i].nets[j].consumer {
                        //вывод знака в 5 столбец что потребитель
                        worksheet.write(_row_point, 4, "+")?;
                        //вывод тока потребления
                        worksheet.write(
                            _row_point,
                            5,
                            x_connector[i].nets[j].current.to_string().clone(),
                        )?;
                    }
                    //вывод цепи земли разъёма по умолчанию
                    //если не установлена по умолчанию
                    if x_connector[i].nets[j].ground_net_manual.as_str() == "".to_string() {
                        worksheet.write(_row_point, 6, "Не установлена по умолчанию")?;
                    } else {
                        worksheet.write(
                            _row_point,
                            6,
                            x_connector[i].nets[j].ground_net_manual.clone(),
                        )?;
                    }
                }
                if x_connector[i].nets.len() - 1 != j {
                    _row_point += 1;
                }
            }
        }
    }
    // Write a number to cell (1, 0) = A2.
    worksheet.autofit();
    let mut _path: String = String::new();
    //если из файла взята стопка с разъёмами
    if mode == "xlsx" {
        _path = format!(
            "./test/data/allegro/x/{}_из_xls.xlsx",
            path_name_spd.clone()
        );
        // println!("{}", path.clone());
    }
    //если из .spd
    else {
        _path = format!(
            "./test/data/allegro/x/{}_из_spd.xlsx",
            path_name_spd.clone()
        );
    }
    //println!("путь xlsx: ./test/sigrity/{}.xlsx", path_x_find);
    // Save the file to disk.
    workbook.save(_path)?;
    */
    let _path: String = format!(
            "./end/2.xlsx",
            
        );
    worksheet.autofit();
    workbook.save(_path)?;
    Ok(())
}