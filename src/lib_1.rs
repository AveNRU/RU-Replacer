use regex::Regex;

//Стопка с путём до книги и содержимым виде вектора строк
#[derive(Debug, Default, Clone)]
pub struct Books {
    pub path: String,         //путь до книги
    pub name: String,         //имя книги
    pub content: Vec<String>, //содержимое
    pub format: String,       //формат
}

//словарь
#[derive(Debug, Default, Clone)]
pub struct Dictionary {
    pub path: String,                      //путь до книги
    pub name: String,                      //имя книги
    pub format: String,                    //формат
    pub single: Vec<String>,               //одиночные слова
    pub re_single: Vec<Regex>,             //одиночные слова Regex
    pub change_single: Vec<String>,        //замена одиночные слова
    pub complex: Vec<String>,              //сложные и составные
    pub re_complex: Vec<Regex>,            //сложные и составные Regex
    pub change_complex: Vec<String>,       //сложные и составные
    pub complex_first: Vec<String>,        //сложные и составные (в 1 очередь)
    pub re_complex_first: Vec<Regex>,      //сложные и составные Regex (в 1 очередь)
    pub change_complex_first: Vec<String>, //сложные и составные (в 1 очередь)
    pub everywhere: Vec<String>,           //сложные и составные
    pub re_everywhere: Vec<Regex>,         //сложные и составные Regex
    pub change_everywhere: Vec<String>,    //сложные и составные
}

//случаи замены

#[derive(Debug, Default, Clone)]
pub struct ChangeWordsSuccess {
    pub single: Vec<String>,         //одиночные слова
    pub change_single: Vec<String>,  //замена одиночные слова
    pub count_single: Vec<usize>,    //количество замен
    pub complex: Vec<String>,        //сложные и составные
    pub change_complex: Vec<String>, //сложные и составные
    pub count_complex: Vec<usize>,   //количество замен
}

//итоговый общий словарь
#[derive(Debug, Default, Clone)]
pub struct FullDictionary {
    //одиночные
    pub single: Vec<String>,               //одиночные слова
    pub re_single: Vec<Regex>,             //одиночные слова Regex
    pub change_single: Vec<String>,        //замена одиночные слова
    pub single_replace_count: Vec<usize>,        //количество замен одиночных слов
    //сложные
    pub complex: Vec<String>,              //сложные и составные
    pub re_complex: Vec<Regex>,            //сложные и составные Regex
    pub change_complex: Vec<String>,       //сложные и составные
    pub complex_replace_count: Vec<usize>, //количество замен сложных и составных слов
    //сложные в 1 очередь
    pub complex_first: Vec<String>,        //сложные и составные (в 1 очередь)
    pub re_complex_first: Vec<Regex>,      //сложные и составные Regex (в 1 очередь)
    pub change_complex_first: Vec<String>, //сложные и составные (в 1 очередь)
    pub complex_first_replace_count: Vec<usize>, //количество замен сложных и составных слов (в 1 очередь)
    //вездесущие слова в 1 очередь
    pub everywhere: Vec<String>,           //сложные и составные
    pub re_everywhere: Vec<Regex>,         //сложные и составные Regex
    pub change_everywhere: Vec<String>,    //сложные и составные
    pub everywhere_replace_count: Vec<usize>, //количество замен сложных и составных слов (в 1 очередь)
}