
use crate::lib_1::{self, Dictionary};
use crate::write::{self};
use calamine::{Data, Reader, Xlsx, open_workbook};
use convert_case::{Case, Casing};
use regex::Regex;
//загрузка словаря
pub fn import_dictionary(dictionary_path_vec: &Vec<String>) -> Vec<lib_1::Dictionary> {
    //итоговая стопка
    let mut _dictionary_vec: Vec<lib_1::Dictionary> = Vec::new();
    for i in 0..dictionary_path_vec.len() {
        //пустая стопка
        let mut _dictionary: lib_1::Dictionary = Dictionary {
            path: dictionary_path_vec[i].clone(), //путь
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
            //запрос на слово искомое в строке
            let word_1 = xlsx_row_value(&main_words, k, 0 as usize);
            //запрос на слово замены в строке
            let word_2 = xlsx_row_value(&main_words, k, 1 as usize);
            //вложение искомого слова
            if !word_1.is_empty() {
                //вложение в общий список для сравнения
                _all_word_find_vec.push(word_1.clone());
                //println!("изначально{}",&word_1);
                //все буквы нижние
                let _s_lowercase: String = word_1.to_case(Case::Lower);
                //println!("нижний : {}",&_s_lowercase);
                //1-я буква заглавная
                let _s_sentence: String = word_1.to_case(Case::Sentence);
                //println!("заголовок: {}",&_s_sentence);
                //вложение когда все буквы нижние
                _word_find_vec.push(_s_lowercase);
                //вложение когда 1-я буква заглавная
                _word_find_vec.push(_s_sentence);
            } else {
                if !word_2.is_empty() {
                    println!(
                        "Ячейка в словаре (1 стр) искомых слов пустая, её номер: {}, но ячейка со словом-заменой содержит не пустое значение ",
                        k
                    );
                }
            }
            //вложение замены
            if !word_2.is_empty() {
                //1-я буква нижняя
                let _s_lowercase: String = word_2.to_case(Case::Lower);
                //println!("нижний : {}",&_s_lowercase);
                //1-я буква заглавная
                let _s_sentence: String = word_2.to_case(Case::Sentence);
                //println!("заголовок: {}",&_s_sentence);
                //вложение когда все буквы нижние
                _word_change_vec.push(_s_lowercase);
                //вложение когда 1-я буква заглавная
                _word_change_vec.push(_s_sentence);
            } else {
                if !word_1.is_empty() {
                    println!(
                        "Ячейка в словаре (1 стр) замен пустая, её номер: {} , но ячейка с искомым словом содержит не пустое значение",
                        k
                    );
                }
            }
        }
        //поиск уже добавленных слов
        for i in 0.._all_word_find_vec.len() {
            //второй круговорот
            for j in i + 1.._all_word_find_vec.len() {
                if _all_word_find_vec[i].as_str() == _all_word_find_vec[j].as_str() {
                    println!(
                        "слово в словаре (1 стр): |{}| уже добавлено. Номер строки 1){i} , 2){j}",
                        &_all_word_find_vec[i]
                    );
                }
            }
        }
        //проверка что количество слов равно
        if _word_find_vec.len() != _word_change_vec.len() {
            println!(
                "Не равно количество слов (1 стр) искомых: {} и замен: {}",
                _word_find_vec.len(),
                _word_change_vec.len()
            );
        }
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
                //запрос на слово искомое в строке
                let word_1 = xlsx_row_value(&complex_words, k, 0 as usize);
                //запрос на слово замены в строке
                let word_2 = xlsx_row_value(&complex_words, k, 1 as usize);
                //вложение искомого слова
                if !word_1.is_empty() {
                    //вложение в общий список для сравнения
                    _all_word_find_vec.push(word_1.clone());
                    //println!("изначально{}",&word_1);
                    //все буквы нижние
                    let _s_lowercase: String = word_1.to_case(Case::Lower);
                    //println!("нижний : {}",&_s_lowercase);
                    //1-я буква заглавная
                    let _s_sentence: String = word_1.to_case(Case::Sentence);
                    //println!("заголовок: {}",&_s_sentence);
                    //вложение когда все буквы нижние
                    _word_find_vec.push(_s_lowercase);
                    //вложение когда 1-я буква заглавная
                    _word_find_vec.push(_s_sentence);
                } else {
                    if !word_2.is_empty() {
                        println!(
                            "Ячейка в словаре (2 стр) искомых слов пустая, её номер: {}, но ячейка со словом-заменой содержит не пустое значение ",
                            k
                        );
                    }
                }
                //вложение замены
                if !word_2.is_empty() {
                    //1-я буква нижняя
                    let _s_lowercase: String = word_2.to_case(Case::Lower);
                    //println!("нижний : {}",&_s_lowercase);
                    //1-я буква заглавная
                    let _s_sentence: String = word_2.to_case(Case::Sentence);
                    //println!("заголовок: {}",&_s_sentence);
                    //вложение когда все буквы нижние
                    _word_change_vec.push(_s_lowercase);
                    //вложение когда 1-я буква заглавная
                    _word_change_vec.push(_s_sentence);
                } else {
                    if !word_1.is_empty() {
                        println!(
                            "Ячейка в словаре (2 стр) замен пустая, её номер: {} , но ячейка с искомым словом содержит не пустое значение",
                            k
                        );
                    }
                }
            }
            //поиск уже добавленных слов
            for i in 0.._all_word_find_vec.len() {
                //второй круговорот
                for j in i + 1.._all_word_find_vec.len() {
                    if _all_word_find_vec[i].as_str() == _all_word_find_vec[j].as_str() {
                        println!(
                            "слово в словаре (2 стр): |{}| уже добавлено. Номер строки 1){i} , 2){j}",
                            &_all_word_find_vec[i]
                        );
                    }
                }
            }
            //проверка что количество слов равно
            if _word_find_vec.len() != _word_change_vec.len() {
                println!(
                    "Не равно количество слов (2 стр) искомых: {} и замен: {}",
                    _word_find_vec.len(),
                    _word_change_vec.len()
                );
            }
            //если это 1 страница
            //if j==0 {
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
            //открытие страницы j
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
                //запрос на слово искомое в строке
                let word_1 = xlsx_row_value(&everywhere_words, k, 0 as usize);
                //запрос на слово замены в строке
                let word_2 = xlsx_row_value(&everywhere_words, k, 1 as usize);
                //вложение искомого слова
                if !word_1.is_empty() {
                    //вложение в общий список для сравнения
                    _all_word_find_vec.push(word_1.clone());
                    //println!("изначально{}",&word_1);
                    //все буквы нижние
                    let _s_lowercase: String = word_1.to_case(Case::Lower);
                    //println!("нижний : {}",&_s_lowercase);
                    //1-я буква заглавная
                    let _s_sentence: String = word_1.to_case(Case::Sentence);
                    //println!("заголовок: {}",&_s_sentence);
                    //вложение когда все буквы нижние
                    _word_find_vec.push(_s_lowercase);
                    //вложение когда 1-я буква заглавная
                    _word_find_vec.push(_s_sentence);
                } else {
                    if !word_2.is_empty() {
                        println!(
                            "Ячейка в словаре (3 стр) искомых слов пустая, её номер: {}, но ячейка со словом-заменой содержит не пустое значение ",
                            k
                        );
                    }
                }
                //вложение замены
                if !word_2.is_empty() {
                    //1-я буква нижняя
                    let _s_lowercase: String = word_2.to_case(Case::Lower);
                    //println!("нижний : {}",&_s_lowercase);
                    //1-я буква заглавная
                    let _s_sentence: String = word_2.to_case(Case::Sentence);
                    //println!("заголовок: {}",&_s_sentence);
                    //вложение когда все буквы нижние
                    _word_change_vec.push(_s_lowercase);
                    //вложение когда 1-я буква заглавная
                    _word_change_vec.push(_s_sentence);
                } else {
                    if !word_1.is_empty() {
                        println!(
                            "Ячейка в словаре (3 стр) замен пустая, её номер: {} , но ячейка с искомым словом содержит не пустое значение",
                            k
                        );
                    }
                }
            }
            //поиск уже добавленных слов
            for i in 0.._all_word_find_vec.len() {
                //третий круговорот
                for j in i + 1.._all_word_find_vec.len() {
                    if _all_word_find_vec[i].as_str() == _all_word_find_vec[j].as_str() {
                        println!(
                            "слово в словаре (3 стр): |{}| уже добавлено. Номер строки 1){i} , 2){j}",
                            &_all_word_find_vec[i]
                        );
                    }
                }
            }
            //проверка что количество слов равно
            if _word_find_vec.len() != _word_change_vec.len() {
                println!(
                    "Не равно количество слов (3 стр) искомых: {} и замен: {}",
                    _word_find_vec.len(),
                    _word_change_vec.len()
                );
            }
            //если это 1 страница
            //if j==0 {
            //вложение искомых слов
            _dictionary.everywhere.extend(_word_find_vec);
            //вложение замены
            _dictionary.change_everywhere.extend(_word_change_vec);
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
            let _re_time: Regex = Regex::new(&_s).unwrap();
            //вложение в вектор искомых слов
            dictionary_lib[i].re_single.push(_re_time);
        }
        //составные слова
        for j in 0..dictionary_lib[i].complex.len() {
            //создание временной строки для искомого слова в соответствии с Regex требованиями (начало и конец слова должен быть)
            let _s: String = format!(r#"\<{}\>"#, dictionary_lib[i].complex[j].clone());
            //создание Regex
            let _re_time: Regex = Regex::new(&_s).unwrap();
            //вложение в вектор искомых слов
            dictionary_lib[i].re_complex.push(_re_time);
        }
        //вездесущие слова
        for j in 0..dictionary_lib[i].everywhere.len() {
            //создание временной строки для искомого слова в соответствии с Regex требованиями (начало и конец слова должен быть)
            let _s: String = format!(r#"{}"#, dictionary_lib[i].everywhere[j].clone());
            //создание Regex
            let _re_time: Regex = Regex::new(&_s).unwrap();
            //вложение в вектор искомых слов
            dictionary_lib[i].re_everywhere.push(_re_time);
        }
        //вывод его
        let _ = write::excel_dictionary_write(&dictionary_lib);
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
