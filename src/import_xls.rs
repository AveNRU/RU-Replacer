 use calamine::{open_workbook, Data, Reader, Xlsx};
use crate::lib_1::{self, Dictionary};
use convert_case::{Case, Casing};
//загрузка словаря
pub fn import_dictionary(dictionary_path_vec:&Vec<String>) ->Vec<lib_1::Dictionary> {
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
    //for j in 0..name_vec_sheets.len() {
        //вектор слов
        let mut _words_1_vec:Vec<String>=Vec::new();
        //вектор замен
        let mut _words_2_vec:Vec<String>=Vec::new();
        //получение имени страницы
        //берём строго 0 страницу
    let name_list = name_vec_sheets[0].clone(); //имя главной страницы
    //открытие страницы книги
    let main_list: Result<calamine::Range<Data>, calamine::XlsxError> =
        workbook.worksheet_range(&name_list); 
        //открытие страницы j
        let main_words: calamine::Range<Data> =
        main_list.expect("не получилось открыть главную страницу в файле .xlsx ");
        //получение значения последней ячейки (строка)
        let last_row = main_words.get_size().0;
        println!("Последняя строка в словаре: {}", &last_row);
        //куда образцы слов вкладываются
        let mut _word_find_vec:Vec<String>=Vec::new();
        //куда замены вставляются
        let mut _word_change_vec:Vec<String>=Vec::new();
        //перебор всех слов
        for k in 0..last_row {
            //запрос на слово искомое в строке
            let word_1 = xlsx_row_value(&main_words, k, 0 as usize);
           
            //вложение искомого слова
            if !word_1.is_empty() {
                //println!("изначально{}",&word_1);
                //все буквы нижние
                let _s_lowercase:String=word_1.to_case(Case::Lower);
                //println!("нижний : {}",&_s_lowercase);
                //1-я буква заглавная
                let _s_sentence:String=word_1.to_case(Case::Sentence);
                //println!("заголовок: {}",&_s_sentence);
                //вложение когда все буквы нижние
                _word_find_vec.push(_s_lowercase);
                //вложение когда 1-я буква заглавная
                _word_find_vec.push(_s_sentence);
            }
            
            //запрос на слово замены в строке
            let word_2 = xlsx_row_value(&main_words, k, 1 as usize);
            //вложение замены
            if !word_2.is_empty() {
                 //1-я буква нижняя
                let _s_lowercase:String=word_2.to_case(Case::Lower);
                //println!("нижний : {}",&_s_lowercase);
                //1-я буква заглавная
                let _s_sentence:String=word_2.to_case(Case::Sentence);
                //println!("заголовок: {}",&_s_sentence);
                //вложение когда все буквы нижние
                _word_change_vec.push(_s_lowercase);
                //вложение когда 1-я буква заглавная
                _word_change_vec.push(_s_sentence);
            }
        }
        //если это 1 страница
        //if j==0 {
            //вложение искомых слов
            _dictionary.single.extend(_word_find_vec);
            //вложение замены
            _dictionary.change_single.extend(_word_change_vec);
       // }
   // }
    //
    
    //вложение в общую стопку
    _dictionary_vec.push(_dictionary);
}
    return _dictionary_vec
}



//получение значения из строки, где i - номер строки, j - номер столбца
pub fn xlsx_row_value(main_components: &calamine::Range<Data>, i: usize, j: usize) -> String {
    let row_str = match main_components.get_value((i as u32, j as u32)) {
        None => "остальное".to_string(),
        Some(Data::Empty) => "".to_string(),
        Some(Data::String(v)) => v.to_string(),
        Some(v) => v.to_string(),
        //_ => "остальное".to_string(),
    };
    // println!("excel:{}",&row_str);
    return row_str.trim().to_string();
    /*  if re_str_maybe_dw(&row_str) {
            return row_str.trim().to_string();
        } else {
            println!(
            "В .xlsx файле ./allegro/config/ недопустимое значение ячейки: |{}|. Номер строки: {}, номер столбца: {}",
            &row_str, i+1,j+1
        );
            system_pause();
            panic!("Аварийное завершение работы...")
        }
    */
}