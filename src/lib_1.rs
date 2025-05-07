//Стопка с путём до книги и содержимым виде вектора строк
#[derive(Debug, Default, Clone)]
pub struct Books {
    pub path: String, //путь до книги
    pub name: String, //имя книги
    pub content: Vec<String>, //содержимое
}