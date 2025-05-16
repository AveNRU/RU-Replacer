//use std::default;

use crate::lib_1::{self, Dictionary,FullDictionary};
//use lazy_static::lazy_static;
//use std::collections::HashMap;
use crate::write::{self};
use regex::Regex;

//изменение слов в книге
pub fn change_words_in_books(
    dictionary_lib: &Vec<Dictionary>,          //вектор словарей
    books_struct_original: &Vec<lib_1::Books>, //книги для изменения
) -> Vec<lib_1::Books> {
    let mut books_struct_changed: Vec<lib_1::Books> = Vec::new();
    //случаи замены слов
    let mut _change_result: lib_1::ChangeWordsSuccess = { Default::default() };
    //создание словаря regex
    let mut full_dictionary: FullDictionary=add_all_words_to_one_dictionary(& dictionary_lib);

    //начало замены слов
    //перебор книг
    for i in 0..books_struct_original.len() {
        //временный вектор для хранения слов
        let mut _time_content: Vec<String> = books_struct_original[i].content.clone();
        //перебор всего содержимого

        //сначала меняются 1)составные (в 1 очередь), 2)вездесущие; 3)сложные слова 4)простые
        for j in 0.._time_content.len() {
            //перебор искомых слов в виде RegEx

            //сложные  слова (в 1 очередь)
            for k in 0..full_dictionary.re_complex_first.len() {
                if full_dictionary.re_complex_first[k].is_match(&_time_content[j]) {
                    //вложение замены во временную переменную
                    let _s: std::borrow::Cow<'_, str> = full_dictionary.re_complex_first[k]
                        .replace_all(&_time_content[j], &full_dictionary.change_complex_first[k]);
                    _time_content[j] = _s.to_string();
                    //увеличение количества совпадений
                    full_dictionary.complex_first_replace_count[k]+=1;
                }
            }

            //вездесущие слова
            for k in 0..full_dictionary.re_everywhere.len() {
                if full_dictionary.re_everywhere[k].is_match(&_time_content[j]) {
                    //вложение замены во временную переменную
                    let _s: std::borrow::Cow<'_, str> =
                        full_dictionary.re_everywhere[k].replace_all(&_time_content[j], &full_dictionary.change_everywhere[k]);
                    _time_content[j] = _s.to_string();
                    //увеличение количества совпадений
                    full_dictionary.everywhere_replace_count[k]+=1;
                }
            }

            //сложные слова
            for k in 0..full_dictionary.re_complex.len() {
                if full_dictionary.re_complex[k].is_match(&_time_content[j]) {
                    //вложение замены во временную переменную
                    let _s: std::borrow::Cow<'_, str> =
                        full_dictionary.re_complex[k].replace_all(&_time_content[j], &full_dictionary.change_complex[k]);
                    _time_content[j] = _s.to_string();
                    //увеличение количества совпадений
                    full_dictionary.complex_replace_count[k]+=1;
                }
            }
            //простые слова
            for k in 0..full_dictionary.re_single.len() {
                if full_dictionary.re_single[k].is_match(&_time_content[j]) {
                    //вложение замены во временную переменную
                    let _s: std::borrow::Cow<'_, str> =
                        full_dictionary.re_single[k].replace_all(&_time_content[j], &full_dictionary.change_single[k]);
                    _time_content[j] = _s.to_string();
                    //увеличение количества совпадений
                    full_dictionary.single_replace_count[k]+=1;
                }
            }
        }
        //вложение книги в стопку новую
        let _time_book = lib_1::Books {
            content: _time_content,
            path: books_struct_original[i].path.clone(),
            name: books_struct_original[i].name.clone(),
            format: books_struct_original[i].format.clone(),
        };
        //вложение в общую стопку
        books_struct_changed.push(_time_book);
        
    }
    //вывод общего словаря
    let _ = write::excel_full_dictionary_write(&full_dictionary);
    return books_struct_changed;
}

pub fn add_all_words_to_one_dictionary ( dictionary_lib: &Vec<Dictionary>,          //вектор словарей
) ->FullDictionary {
    //итоговый словарь
    let mut full_dictionary:FullDictionary={Default::default()};
    //перебор словаря
    for i in 0..dictionary_lib.len() {
        //вездесущие слова
        for j in 0..dictionary_lib[i].everywhere.len() {
            //вложение в вектор искомых слов
            full_dictionary.re_everywhere.push(dictionary_lib[i].re_everywhere[j].clone());
            //вложение в вектор изначальных слов
            full_dictionary.everywhere.push(dictionary_lib[i].everywhere[j].clone());
            //вложение замен
            full_dictionary.change_everywhere.push(dictionary_lib[i].change_everywhere[j].clone());

        }
    
        //составные слова
        for j in 0..dictionary_lib[i].complex.len() {
            //вложение в вектор искомых слов
            full_dictionary.re_complex.push(dictionary_lib[i].re_complex[j].clone());
            //вложение в вектор изначальных слов
            full_dictionary.complex.push(dictionary_lib[i].complex[j].clone());
            //вложение замен
            full_dictionary.change_complex.push(dictionary_lib[i].change_complex[j].clone());
        }

        //составные слова (в 1 очередь)
        for j in 0..dictionary_lib[i].complex_first.len() {
            //вложение в вектор искомых слов
            full_dictionary.re_complex_first.push(dictionary_lib[i].re_complex_first[j].clone());
            //вложение в вектор изначальных слов
            full_dictionary.complex_first.push(dictionary_lib[i].complex_first[j].clone());
            //вложение замен
            full_dictionary.change_complex_first.push(dictionary_lib[i].change_complex_first[j].clone());
        }
        //простые слова
        //перебор искомых слов под замену
        for j in 0..dictionary_lib[i].single.len() {
            //вложение в вектор искомых слов
            full_dictionary.re_single.push(dictionary_lib[i].re_single[j].clone());
            //вложение в вектор изначальных слов
            full_dictionary.single.push(dictionary_lib[i].single[j].clone());
            //вложение замен
            full_dictionary.change_single.push(dictionary_lib[i].change_single[j].clone());
        }
    }
    //установка значений замен по 0
    full_dictionary.single_replace_count.resize(full_dictionary.single.len(), 0);
    full_dictionary.complex_replace_count.resize(full_dictionary.complex.len(), 0);
    full_dictionary.complex_first_replace_count.resize(full_dictionary.complex_first.len(), 0);
    full_dictionary.everywhere_replace_count.resize(full_dictionary.everywhere.len(), 0);
    return full_dictionary
}
