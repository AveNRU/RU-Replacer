pub fn check_file_exists_1() {
    use std::fs::{
        self,
        //metadata, File
    };
    use std::path::Path;
    //двумерный вектор. 1-й для хранения прямых путей, 2 - после пути добавляется косая черта / (linux)
    let mut path_vec: Vec<Vec<String>> = vec![
        vec![
            //test
            "./books".to_string(),
            "./dictionary".to_string(),
            "./end".to_string(),
            "./end/dictionary".to_string(),
        ],
        //пустой изначально для хранения пустых черт
        vec![],
    ];
    for i in 0..path_vec[0].len() {
        let s: String = format!("{}/", path_vec[0][i].clone());
        path_vec[1].push(s);
    }
    for i in 0..path_vec.len() {
        for j in 0..path_vec[i].len() {
            //проверка наличия папок
            if !Path::new(path_vec[i][j].as_str()).exists() {
                let _ = fs::create_dir(path_vec[i][j].as_str()); // создаем папку
            }
            //чтение содержимого папки
            match fs::read_dir(path_vec[i][j].as_str()) {
                //если ошибка - вывод почему
                Err(why) => println!("! {:?}", why.kind()),
                //если успех - получение списка содержимого
                Ok(paths) => {
                    for _path in paths {
                        //   println!("{:?}", path.unwrap().path());
                    }
                }
            }
        }
    }
}
