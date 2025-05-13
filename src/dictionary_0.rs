//use std::default;

use crate::lib_1::{self, 
    Dictionary
};
//use lazy_static::lazy_static;
//use std::collections::HashMap;
use crate::write::{self};
use regex::Regex;

//изменение слов в книге
pub fn change_words_in_books(
    dictionary_lib:&Vec<Dictionary> ,//сам словарь готовый
    books_struct_original: &Vec<lib_1::Books>,//книги для изменения
) -> Vec<lib_1::Books>{
let mut books_struct_changed: Vec<lib_1::Books> = Vec::new();
//изначальные слова одиночные
let mut single_word:Vec<String>=Vec::new();
//изначальные слова сложные
let mut _complex_word:Vec<String>=Vec::new();
//одиночные слова Regex
let mut re_single:Vec<Regex> = Vec::new();
//одиночные слова Regex
let mut re_complex:Vec<Regex> = Vec::new();
//замена - одиночные
let mut change_single_word:Vec<String>=Vec::new();
//замена - множественные
let mut change_complex_word:Vec<String>=Vec::new();
//случаи замены слов
let mut change_result: lib_1::ChangeWordsSuccess={Default::default()};
//создание словаря regex
/*let _s:String="МНого Василий работает".to_string();
let re_1:Regex=Regex::new(r"\bВасилий\b").unwrap();
let _s3:String="Дмитрий".to_string();
let _s2=re_1.replace_all(&_s,_s3);
println!("{}",_s2);
*/
//перебор словаря
for i in 0..dictionary_lib.len() {
    //перебор искомых слов под замену
    for j in 0..dictionary_lib[i].single.len() {
        //создание временной строки для искомого слова в соответствии с Regex требованиями (начало и конец слова должен быть)
    let _s: String=format!(r#"\<{}\>"#,dictionary_lib[i].single[j].clone());
    //создание Regex
    let _re_time:Regex=Regex::new(&_s).unwrap();
    //вложение в вектор искомых слов
    re_single.push(_re_time);
    //вложение в вектор изначальных слов
    single_word.push(dictionary_lib[i].single[j].clone());
    //вложение замен
    change_single_word.push(dictionary_lib[i].change_single[j].clone());
    //замены извлечение
     // for j in 0..dictionary_lib[i].change_single.len() {
        //извлечение замен
       // change_single_word.push(dictionary_lib[i].change_single[j].clone());
     // }
}
}
//for i in 0..re_single.len() {
    //println!("слово: {}",&re_single[i]);
//}
//начало замены слов
//перебор книг
for i in 0..books_struct_original.len() {
//временный вектор для хранения слов
let mut _time_content:Vec<String>=books_struct_original[i].content.clone();
//перебор всего содержимого
for j in 0.._time_content.len() {
     //if _time_content[j].contains("криминальная хроника страны ежедневно") {println!("{j}")}
    //перебор искомых слов в виде RegEx
    
    for k in 0..re_single.len() {
         if j==572 {
          //  println!("до: k: {k} : {}",&_time_content[j]);
           }
        //замена самих слов
       /*
        println!("k: {}",k);
         if re_single[k].is_match(&_time_content[j]) {
            println!("нашло замену : {}",&_time_content[j]);
            let _ss=re_single[k].replace_all(&books_struct_original[i].content[j],&change_single_word[k]).to_string();
            println!("замена: {}",_ss);
         }
       }*/
        if re_single[k].is_match(&_time_content[j]) {
           // println!("Нашло: {}",&re_single[k]);
           
           //вложение замены во временную переменную
        let _s: std::borrow::Cow<'_, str>=re_single[k].replace_all(&_time_content[j],&change_single_word[k]);
        if j==572 && k==0 {
          //  println!("нашло замену : {}",&_s);
           }
        _time_content[j]=_s.to_string();
        if j==572 && k==0 {
           // println!("после замены : {}",&_time_content[j]);
           }
        //let _s=&books_struct_original[i].content[j].clone();
        //_time_content[j]=_s.to_string();
        //_time_content.push(_s.to_string());
         if j==572 {
          //  println!("после: k: {k} : {}",&_time_content[j]);
           }
        } 
        //let _s: std::borrow::Cow<'_, str>=re_single[k].replace_all(&books_struct_original[i].content[j],&change_single_word[k]);
        //вложение в вектор
        //println!("{}",_s);
        
    }
  
}
//вложение книги в стопку новую
let _time_book=lib_1::Books {
    content:_time_content,
    path:books_struct_original[i].path.clone(),
    name:books_struct_original[i].name.clone(),
    format:books_struct_original[i].format.clone(),
};
//вложение в общую стопку
books_struct_changed.push(_time_book);
}

let _time_dictionary:lib_1::Dictionary=lib_1::Dictionary{
    single:single_word,//изначальные слова одиночные
    re_single:re_single,//одиночные слова Regex
    change_single:change_single_word,
    complex:Default::default(),//замена - одиночные
    change_complex:change_complex_word,//замена - множественные
    re_complex:re_complex,//одиночные слова Regex
    path:Default::default(),
    name:Default::default(),
    format:Default::default(),
    //Default::default()
};
//временный вектор со словарями
let _time_dictionary_vec:Vec<lib_1::Dictionary> = vec![_time_dictionary];
//вывод его
let _ = write::excel_dictionary_write(&_time_dictionary_vec);
return books_struct_changed
}

//pub fn 