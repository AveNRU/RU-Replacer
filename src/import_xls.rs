use crate::lib_1::{self, Dictionary};
//загрузка словаря
pub fn import_dictionary(dictionary_path_vec:&Vec<String>) ->Vec<lib_1::Dictionary> {
    use calamine::{open_workbook, Data, Reader, Xlsx};
    //итоговая стопка
    let mut _dictionary_vec:Vec<lib_1::Dictionary> =Vec::new();
    for i in 0..dictionary_path_vec.len() {
        //пустая стопка
    let mut _dictionary:lib_1::Dictionary=Dictionary{
        path:dictionary_path_vec[i].clone(),//путь
        ..Default::default()
    };
    //начало
    let mut workbook: Xlsx<_> =
    open_workbook(&dictionary_path_vec[i]).expect("Не могу открыть файл .xlsx в папке ./allegro/config");
    let name_vec_sheets= workbook.sheet_names();
    //чтение содержимого всех страниц
    for j in 0..name_vec_sheets.len() {
        //вектор слов
        let mut _words_1_vec:Vec<String>=Vec::new();
        //вектор замен
        let mut _words_2_vec:Vec<String>=Vec::new();
        //получение имени страницы
    let name_list = name_vec_sheets[j].clone(); //имя главной страницы
    //открытие страницы книги
    let main_list: Result<calamine::Range<Data>, calamine::XlsxError> =
        workbook.worksheet_range(&name_list); 
        let main_words: calamine::Range<Data> =
        main_list.expect("не получилось открыть главную страницу в файле .xlsx ");
        let last_row = main_words.get_size().0;
        for k in 0..last_row {
            let word_1 = xlsx_row_value(&main_words, k, 0 as usize);
            let word_2 = xlsx_row_value(&main_words, k, 1 as usize);
        }

    }
    //
    
    //вложение в общую стопку
    _dictionary_vec.push(_dictionary);
}
    return _dictionary_vec
}