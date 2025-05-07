//Стопка с путём до книги и содержимым виде вектора строк
#[derive(Debug, Default, Clone)]
pub struct Books {
    pub path: String, //путь до книги
    pub name: String, //имя книги
    pub content: Vec<String>, //содержимое
    pub format: String,//формат
}

//словарь
#[derive(Debug, Default, Clone)]
pub struct Dictionary {
    pub path: String, //путь до книги
    pub name: String, //имя книги
    pub format: String,//формат
    pub single:Vec<Vec<String>>,//одиночные слова
    pub comples:Vec<Vec<String>>,//сложные и составные
}