use crate::lib_1::{self, Dictionary};
use crate::write::{self};
use calamine::{Data, Reader, Xlsx, open_workbook};
use convert_case::{Case, Casing};
use regex::Regex;
//загрузка словаря
pub fn import_dictionary(dictionary_path_vec: &Vec<String>) -> Vec<lib_1::Dictionary> {
    //имя словаря вырезать

    let re_name_dictionary_vec:Vec<Regex> = vec![
        Regex::new(r"(?i)\\([\d\w\s]+)\.(?:([\d\w_-]+))$").unwrap(),
        Regex::new(r"(?i)/([\d\w\s]+)\.(?:([\d\w_-]+))$").unwrap(),
        ];
    //итоговая стопка
    let mut _dictionary_vec: Vec<lib_1::Dictionary> = Vec::new();
    for i in 0..dictionary_path_vec.len() {
        
        //пустая строка под имя словаря
        let mut _name_dictionary:String=String::new();
        //перебор устойчивых образцов
        for k in 0..re_name_dictionary_vec.len() {
        //если успешно выделение имени файла
        if let Some(caps) = re_name_dictionary_vec[k].captures(&&dictionary_path_vec[i]) {
                _name_dictionary = caps[1].trim().to_string();
                //println!("refdes1");
            } else {
                _name_dictionary=format!("Словарь_№_{}",i);
            }
        }
        //вывод имени словаря
        println!("Словарь №{i}: {}",&_name_dictionary);
        //пустая стопка
        let mut _dictionary: lib_1::Dictionary = Dictionary {
            path: dictionary_path_vec[i].clone(), //путь
            name:_name_dictionary,
            ..Default::default()
        };
        //начало
        let mut workbook: Xlsx<_> = open_workbook(&dictionary_path_vec[i])
            .expect("Не могу открыть файл .xlsx в папке ./dictionary/");
        let name_vec_sheets = workbook.sheet_names();
        //чтение содержимого всех страниц
        //вектор слов
        let mut _words_1_vec: Vec<String> = Vec::new();
        //вектор замен
        let mut _words_2_vec: Vec<String> = Vec::new();
        //получение имени страницы
        //берём строго 0 страницу
        let name_list = name_vec_sheets[0].clone(); //имя главной страницы
        let main_list: Result<calamine::Range<Data>, calamine::XlsxError> =
            workbook.worksheet_range(&name_list);
        //открытие страницы книги
        //открытие страницы j
        let main_words: calamine::Range<Data> =
            main_list.expect("не получилось открыть главную страницу в файле .xlsx ");
        //получение значения последней ячейки (строка)
        let last_row = main_words.get_size().0;
        println!("Последняя строка на 1 странице в словаре: {}", &last_row);
        // для сравнения
        let mut _all_word_find_vec: Vec<String> = Vec::new();
        //куда образцы слов вкладываются
        let mut _word_find_vec: Vec<String> = Vec::new();
        //куда замены вставляются
        let mut _word_change_vec: Vec<String> = Vec::new();
        //перебор всех слов
        for k in 0..last_row {
            //извлечение слов , переделка их под заглавные буквы и маленькие
            add_word_to_dictionary(
                xlsx_row_value(&main_words, k, 0 as usize),
                xlsx_row_value(&main_words, k, 1 as usize),
                &mut _all_word_find_vec,
                &mut _word_find_vec, //куда образцы слов вкладываются
                &mut _word_change_vec,
                1, //страница
                k, //указатель ячейки
            );
        }
        find_allready_words(
            &mut _all_word_find_vec, // для сравнения
            &mut _word_find_vec,     //куда образцы слов вкладываются
            &mut _word_change_vec,   //куда замены вставляются
            1,                       //номер страницы
        );
        //если это 1 страница
        //вложение искомых слов
        _dictionary.single.extend(_word_find_vec);
        //вложение замены
        _dictionary.change_single.extend(_word_change_vec);
        //второй лист
        let mut second_list_name: String = String::new();
        //если больше чем 1 страница
        if name_vec_sheets.len() >= 1 {
            second_list_name = name_vec_sheets[1].clone(); //имя 2 страницы
        }
        //если не пустое имя страницы
        if !second_list_name.is_empty() {
            //получение страницы
            let second_list: Result<calamine::Range<Data>, calamine::XlsxError> =
                workbook.worksheet_range(&second_list_name);

            //открытие 2 страницы книги
            //открытие страницы j
            let complex_words: calamine::Range<Data> =
                second_list.expect("не получилось открыть вторую (2) страницу в файле .xlsx ");
            //получение значения последней ячейки (строка)
            let last_row = complex_words.get_size().0;
            println!("Последняя строка на 2 странице в словаре: {}", &last_row);
            // для сравнения
            let mut _all_word_find_vec: Vec<String> = Vec::new();
            //куда образцы слов вкладываются
            let mut _word_find_vec: Vec<String> = Vec::new();
            //куда замены вставляются
            let mut _word_change_vec: Vec<String> = Vec::new();
            //перебор всех слов
            for k in 0..last_row {
                add_word_to_dictionary(
                    xlsx_row_value(&complex_words, k, 0 as usize),
                    xlsx_row_value(&complex_words, k, 1 as usize),
                    &mut _all_word_find_vec,
                    &mut _word_find_vec,
                    &mut _word_change_vec,
                    2, //страница
                    k, //указатель ячейки
                );
            }
            find_allready_words(
                &mut _all_word_find_vec, // для сравнения
                &mut _word_find_vec,     //куда образцы слов вкладываются
                &mut _word_change_vec,   //куда замены вставляются
                2,                       //номер страницы
            );
            //вложение искомых слов
            _dictionary.complex.extend(_word_find_vec);
            //вложение замены
            _dictionary.change_complex.extend(_word_change_vec);
            // }
        }

        //третий лист
        let mut third_list_name: String = String::new();
        //если больше чем 1 страница
        if name_vec_sheets.len() >= 2 {
            third_list_name = name_vec_sheets[2].clone(); //имя 3 страницы
        }
        //если не пустое имя страницы
        if !third_list_name.is_empty() {
            //получение страницы
            let third_list: Result<calamine::Range<Data>, calamine::XlsxError> =
                workbook.worksheet_range(&third_list_name);

            //открытие 3 страницы книги
            let everywhere_words: calamine::Range<Data> =
                third_list.expect("не получилось открыть третью (3) страницу в файле .xlsx ");
            //получение значения последней ячейки (строка)
            let last_row = everywhere_words.get_size().0;
            println!("Последняя строка на 3 странице в словаре: {}", &last_row);
            // для сравнения
            let mut _all_word_find_vec: Vec<String> = Vec::new();
            //куда образцы слов вкладываются
            let mut _word_find_vec: Vec<String> = Vec::new();
            //куда замены вставляются
            let mut _word_change_vec: Vec<String> = Vec::new();
            //перебор всех слов
            for k in 0..last_row {
                add_word_to_dictionary(
                    xlsx_row_value(&everywhere_words, k, 0 as usize),
                    xlsx_row_value(&everywhere_words, k, 1 as usize),
                    &mut _all_word_find_vec,
                    &mut _word_find_vec,
                    &mut _word_change_vec,
                    3, //страница
                    k, //указатель ячейки
                );
            }
            find_allready_words(
                &mut _all_word_find_vec, // для сравнения
                &mut _word_find_vec,     //куда образцы слов вкладываются
                &mut _word_change_vec,   //куда замены вставляются
                3,                       //номер страницы
            );
            //вложение искомых слов
            _dictionary.everywhere.extend(_word_find_vec);
            //вложение замены
            _dictionary.change_everywhere.extend(_word_change_vec);
            // }
        }

        //четвертый лист
        let mut four_list_name: String = String::new();
        //если больше или равно чем 4 страницы
        if name_vec_sheets.len() >= 3 {
            four_list_name = name_vec_sheets[3].clone(); //имя 4 страницы
        }
        //если не пустое имя страницы
        if !four_list_name.is_empty() {
            //получение страницы
            let four_list: Result<calamine::Range<Data>, calamine::XlsxError> =
                workbook.worksheet_range(&four_list_name);

            //открытие 4 страницы книги
            let everywhere_words: calamine::Range<Data> =
                four_list.expect("не получилось открыть (4 стр) страницу в файле .xlsx ");
            //получение значения последней ячейки (строка)
            let last_row = everywhere_words.get_size().0;
            println!(
                "Последняя строка на (4 стр) странице в словаре: {}",
                &last_row
            );
            // для сравнения
            let mut _all_word_find_vec: Vec<String> = Vec::new();
            //куда образцы слов вкладываются
            let mut _word_find_vec: Vec<String> = Vec::new();
            //куда замены вставляются
            let mut _word_change_vec: Vec<String> = Vec::new();
            //перебор всех слов
            for k in 0..last_row {
                add_word_to_dictionary(
                    xlsx_row_value(&everywhere_words, k, 0 as usize),
                    xlsx_row_value(&everywhere_words, k, 1 as usize),
                    &mut _all_word_find_vec,
                    &mut _word_find_vec,
                    &mut _word_change_vec,
                    4, //страница
                    k, //указатель ячейки
                );
            }
            find_allready_words(
                &mut _all_word_find_vec, // для сравнения
                &mut _word_find_vec,     //куда образцы слов вкладываются
                &mut _word_change_vec,   //куда замены вставляются
                4,                       //номер страницы
            );
            //вложение искомых слов
            _dictionary.complex_first.extend(_word_find_vec);
            //вложение замены
            _dictionary.change_complex_first.extend(_word_change_vec);
            // }
        }
        //вложение в общую стопку
        _dictionary_vec.push(_dictionary);
    }
    add_re_word_to_dictionary(&mut _dictionary_vec);
    return _dictionary_vec;
}

//добавить слов re в словарь
pub fn add_re_word_to_dictionary(
    dictionary_lib: &mut Vec<Dictionary>, //сам словарь готовый
) -> Vec<lib_1::Dictionary> {
    //пустой вектор словарей
    let mut _dictionary_vec: Vec<lib_1::Dictionary> = Vec::new();
    //перебор словаря
    for i in 0..dictionary_lib.len() {
        //перебор искомых слов под замену
        for j in 0..dictionary_lib[i].single.len() {
            //создание временной строки для искомого слова в соответствии с Regex требованиями (начало и конец слова должен быть)
            let _s: String = format!(r#"\<{}\>"#, dictionary_lib[i].single[j].clone());
            //создание Regex
           // let _re_time: Regex = Regex::new(&_s).unwrap();
            //вложение в вектор искомых слов
            dictionary_lib[i].re_single.push(Regex::new(&_s).unwrap());
        }
        //составные слова
        for j in 0..dictionary_lib[i].complex.len() {
            //создание временной строки для искомого слова в соответствии с Regex требованиями (начало и конец слова должен быть)
            let _s: String = format!(r#"({})"#, dictionary_lib[i].complex[j].clone());
            //создание Regex
           // let _re_time: Regex = Regex::new(&_s).unwrap();
            //вложение в вектор искомых слов
            dictionary_lib[i].re_complex.push(Regex::new(&_s).unwrap());
        }
        //вездесущие слова
        for j in 0..dictionary_lib[i].everywhere.len() {
            //создание временной строки для искомого слова в соответствии с Regex требованиями (начало и конец слова должен быть)
            let _s: String = format!(r#"{}"#, dictionary_lib[i].everywhere[j].clone());
            //создание Regex
           // let _re_time: Regex = Regex::new(&_s).unwrap();
            //вложение в вектор искомых слов
            dictionary_lib[i].re_everywhere.push(Regex::new(&_s).unwrap());
        }
        //составные слова
        for j in 0..dictionary_lib[i].complex_first.len() {
            //создание временной строки для искомого слова в соответствии с Regex требованиями (начало и конец слова должен быть)
            let _s: String = format!(r#"({})"#, dictionary_lib[i].complex_first[j].clone());
            //создание Regex
           // let _re_time: Regex = Regex::new(&_s).unwrap();
            //вложение в вектор искомых слов
            dictionary_lib[i].re_complex_first.push(Regex::new(&_s).unwrap());
        }
        //вывод каждого по отдельности словаря
        //let _ = write::excel_dictionary_write(&dictionary_lib);
    }

    return _dictionary_vec;
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
}
//поиск знаков
//если встречается знак в строке _ либо - , то заменить его на ∷
pub fn find_spec_symbols_str(_exodus: &String) -> (String, Vec<char>) {
    //вектор знаков под строку
    let mut ch_vec: Vec<char> = _exodus.chars().collect();
    //знак возврата
    //∴ ∵ ∷
    //пустой вектор под знаки
    let mut exodus_char_vec: Vec<char> = Vec::new();
    //по умолчанию знак
    //let mut _exodus_char: char = '\x00';

    //замена двоеточий внутри кавычек на особые знаки
    for i in 0..ch_vec.len() {
        // замена особых знаков
        match ch_vec[i] {
            '-' => {
                exodus_char_vec.push('∴');
                ch_vec[i] = '∴'
            } // - меняет на ∴
            '_' => {
                exodus_char_vec.push('∵');
                ch_vec[i] = '∵'
            } // _ меняет на ∵
            _ => (),
        }
    }
    //вложение в строку всех знаков
    return (ch_vec.into_iter().collect(), exodus_char_vec);
}
//обратно перевод знаков
//если встречается знак в строке _ либо - , то заменить его на ∷
pub fn change_spec_symbols_str(word_1: String,word_2:String,char_vec:Vec<char>) -> (String, String) {
    //вектор знаков 1 слова 
    let mut ch_vec_1: Vec<char> = word_1.chars().collect();
     //вектор знаков 2 слова 
    let mut ch_vec_2: Vec<char> = word_2.chars().collect();
    //знак возврата
    //∴ ∵ ∷
    //по умолчанию знак
    //замена двоеточий внутри кавычек на особые знаки
    for i in 0..ch_vec_1.len() {
        // замена особых знаков
        match ch_vec_1[i] {
            '∴' => {
                ch_vec_1[i] = '-'
            } // - меняет на ∴
            '∵' => {
                ch_vec_1[i] = '_'
            } // _ меняет на ∵
            _ => (),
        }
    }
    //замена двоеточий внутри кавычек на особые знаки
    for i in 0..ch_vec_2.len() {
        // замена особых знаков
        match ch_vec_2[i] {
            '∴' => {
                ch_vec_2[i] = '-'
            } // - меняет на ∴
            '∵' => {
                ch_vec_2[i] = '_'
            } // _ меняет на ∵
            _ => (),
        }
    }
//возврат слов
    return (ch_vec_1.into_iter().collect(), ch_vec_2.into_iter().collect());
}

//добавление слов и замен словарь в большими и маленькими буквами
pub fn add_word_to_dictionary(
    word_1: String,                       //1-е слово (искомое)
    word_2: String,                       //2-е слово (замена)
    _all_word_find_vec: &mut Vec<String>, //общий список для сравнения
    _word_find_vec: &mut Vec<String>, //все общее количество слов, где все буквы нижние и первая заглавная
    _word_change_vec: &mut Vec<String>, //замены слов
    number_list: usize,               //номер страницы
    k: usize,                         //номер ячейки
) {
    //запрос на слово искомое в строке
    //вложение искомого слова
    if !word_1.is_empty() {
        //вложение в общий список для сравнения
        _all_word_find_vec.push(word_1.clone());
        let tuple: (String, Vec<char>) = find_spec_symbols_str(&word_1);
        //все буквы нижние
        let _s_lowercase: String = tuple.0.to_case(Case::Lower);
        //1-я буква заглавная
        let _s_sentence: String =  tuple.0.to_case(Case::Sentence);
        //упорядоченный ряд, где производится обратная замена - и _
        let tuple =change_spec_symbols_str(_s_lowercase,_s_sentence,tuple.1);
        //вложение когда все буквы нижние
        _word_find_vec.push(tuple.0);
        //вложение когда 1-я буква заглавная
        _word_find_vec.push(tuple.1);
    } else {
        if !word_2.is_empty() {
            println!(
                "Ячейка в словаре ({} стр) искомых слов пустая, её номер: {}, но ячейка со словом-заменой содержит не пустое значение ",
                number_list, k
            );
        }
    }
    //вложение замены
    if !word_2.is_empty() {
        //1-я буква нижняя
        let tuple: (String, Vec<char>) = find_spec_symbols_str(&word_2);
        //все буквы нижние
        let _s_lowercase: String = tuple.0.to_case(Case::Lower);
        //1-я буква заглавная
        let _s_sentence: String =  tuple.0.to_case(Case::Sentence);
        //упорядоченный ряд, где производится обратная замена - и _
        let tuple =change_spec_symbols_str(_s_lowercase.clone(),_s_sentence.clone(),tuple.1);
        //вложение когда все буквы нижние
        _word_change_vec.push(tuple.0);
        //вложение когда 1-я буква заглавная
        _word_change_vec.push(tuple.1);

    } else {
        if !word_1.is_empty() {
            println!(
                "Ячейка в словаре ({} стр) замен пустая, её номер: {} , но ячейка с искомым словом содержит не пустое значение",
                number_list, k
            );
        }
    }
}

pub fn find_allready_words(
    _all_word_find_vec: &Vec<String>, // для сравнения
    _word_find_vec: &Vec<String>,     //куда образцы слов вкладываются
    _word_change_vec: &Vec<String>,   //куда замены вставляются
    number_list: usize,               //номер страницы
) {
    //поиск уже добавленных слов
    for i in 0.._all_word_find_vec.len() {
        //второй круговорот
        for j in i + 1.._all_word_find_vec.len() {
            if _all_word_find_vec[i].as_str() == _all_word_find_vec[j].as_str() {
                println!(
                    "слово в словаре ({} стр): |{}| уже добавлено. Номер строки 1){i} , 2){j}",
                    number_list, &_all_word_find_vec[i]
                );
            }
        }
    }
    //проверка что количество слов равно
    if _word_find_vec.len() != _word_change_vec.len() {
        println!(
            "Не равно количество слов ({} стр) искомых: {} и замен: {}",
            number_list,
            _word_find_vec.len(),
            _word_change_vec.len()
        );
    }
}
