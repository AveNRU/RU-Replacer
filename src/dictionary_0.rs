//use std::default;

use crate::lib_1::{self, Dictionary};
//use lazy_static::lazy_static;
//use std::collections::HashMap;
//use crate::write::{self};
use regex::Regex;

//изменение слов в книге
pub fn change_words_in_books(
    dictionary_lib: &Vec<Dictionary>,          //сам словарь готовый
    books_struct_original: &Vec<lib_1::Books>, //книги для изменения
) -> Vec<lib_1::Books> {
    let mut books_struct_changed: Vec<lib_1::Books> = Vec::new();

    //изначальные слова одиночные
    let mut single_word: Vec<String> = Vec::new();
    //одиночные слова Regex
    let mut re_single: Vec<Regex> = Vec::new();
    //замена - одиночные
    let mut change_single_word: Vec<String> = Vec::new();

     //изначальные слова сложные
    let mut complex_word: Vec<String> = Vec::new();
    //составные слова Regex
    let mut re_complex: Vec<Regex> = Vec::new();
    //замена - множественные
    let mut change_complex_word: Vec<String> = Vec::new();

    //одиночные слова Regex
    let mut re_everywhere: Vec<Regex> = Vec::new();
    //замена - одиночные
    let mut everywhere_word: Vec<String> = Vec::new();
    //замена - множественные
    let mut change_everywhere_word: Vec<String> = Vec::new();

     //изначальные слова сложные
    let mut complex_first_word: Vec<String> = Vec::new();
    //составные слова Regex
    let mut re_complex_first: Vec<Regex> = Vec::new();
    //замена - множественные
    let mut change_complex_first_word: Vec<String> = Vec::new();
    //случаи замены слов
    let mut _change_result: lib_1::ChangeWordsSuccess = { Default::default() };
    //создание словаря regex

    //перебор словаря
    for i in 0..dictionary_lib.len() {
        //вездесущие слова
        for j in 0..dictionary_lib[i].everywhere.len() {
            //вложение в вектор искомых слов
            re_everywhere.push(dictionary_lib[i].re_everywhere[j].clone());
            //вложение в вектор изначальных слов
            everywhere_word.push(dictionary_lib[i].everywhere[j].clone());
            //вложение замен
            change_everywhere_word.push(dictionary_lib[i].change_everywhere[j].clone());
        }

         //составные слова
        for j in 0..dictionary_lib[i].complex.len() {
            //вложение в вектор искомых слов
            re_complex.push(dictionary_lib[i].re_complex[j].clone());
            //вложение в вектор изначальных слов
            complex_word.push(dictionary_lib[i].complex[j].clone());
            //вложение замен
            change_complex_word.push(dictionary_lib[i].change_complex[j].clone());
        }

         //составные слова (в 1 очередь)
        for j in 0..dictionary_lib[i].complex_first.len() {
            //вложение в вектор искомых слов
            re_complex_first.push(dictionary_lib[i].re_complex_first[j].clone());
            //вложение в вектор изначальных слов
            complex_first_word.push(dictionary_lib[i].complex_first[j].clone());
            //вложение замен
            change_complex_first_word.push(dictionary_lib[i].change_complex_first[j].clone());
        }
        //простые слова
        //перебор искомых слов под замену
        for j in 0..dictionary_lib[i].single.len() {
            //вложение в вектор искомых слов
            re_single.push(dictionary_lib[i].re_single[j].clone());
            //вложение в вектор изначальных слов
            single_word.push(dictionary_lib[i].single[j].clone());
            //вложение замен
            change_single_word.push(dictionary_lib[i].change_single[j].clone());
        }
       
    }

    //начало замены слов
    //перебор книг
    for i in 0..books_struct_original.len() {
        //временный вектор для хранения слов
        let mut _time_content: Vec<String> = books_struct_original[i].content.clone();
        //перебор всего содержимого


        //сначала меняются 1)составные (в 1 очередь), 2)вездесущие; 3)сложные слова 4)простые
        for j in 0.._time_content.len() {
            //перебор искомых слов в виде RegEx

            //сложные  слова
            for k in 0..re_complex_first.len() {
                if re_complex_first[k].is_match(&_time_content[j]) {
                    //вложение замены во временную переменную
                    let _s: std::borrow::Cow<'_, str> =
                        re_complex_first[k].replace_all(&_time_content[j], &change_complex_first_word[k]);
                    _time_content[j] = _s.to_string();
                }
            }

            //вездесущие слова
            for k in 0..re_everywhere.len() {
                if re_everywhere[k].is_match(&_time_content[j]) {
                    //вложение замены во временную переменную
                    let _s: std::borrow::Cow<'_, str> =
                        re_everywhere[k].replace_all(&_time_content[j], &change_everywhere_word[k]);
                    _time_content[j] = _s.to_string();
                }
            }

            //сложные слова
            for k in 0..re_complex.len() {
                if re_complex[k].is_match(&_time_content[j]) {
                    //вложение замены во временную переменную
                    let _s: std::borrow::Cow<'_, str> =
                        re_complex[k].replace_all(&_time_content[j], &change_complex_word[k]);
                    _time_content[j] = _s.to_string();
                }
            }
            //простые слова
            for k in 0..re_single.len() {
                if re_single[k].is_match(&_time_content[j]) {
                    //вложение замены во временную переменную
                    let _s: std::borrow::Cow<'_, str> =
                        re_single[k].replace_all(&_time_content[j], &change_single_word[k]);
                    _time_content[j] = _s.to_string();
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
    return books_struct_changed;
}

//pub fn
