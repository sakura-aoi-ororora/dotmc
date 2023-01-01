use crate::func::Datapack;
use std::fs::{create_dir, File};
use std::io::Write;
use crate::create_mcfunction::create_function;

pub fn create_datapack(datapack: Datapack, path: &str, name: &str) {
    let path = String::from(path);
    let name = String::from(name);

    //create datapack(<name>)
    let path = format!(r"{}\{}", path, name);
    create_dir_mine(&path);

    //todo: pack.pngの追加
    //create <name>\pack.mcmeta
    let meta = format!(r"{}\pack.mcmeta", path);
    let data = String::from(
        r##"{
  "pack": {
    "pack_format": 9,
    "description": "Generate by dotmc"
  }
}"##,
    );

    create_file_mine(&meta, &data);

    //create <name>\data path + r"\data"
    let data_path = path + r"\data"; // pathの所有権を消す
    create_dir_mine(&data_path);

    //todo: 進捗、ルートテーブル、その他諸々の追加
    //todo: 要素の削除系
    if let Some(b) = datapack.function {
        //create <name>\data\minecraft
        let minecraft_path = format!(r"{}\minecraft", data_path);
        create_dir_mine(&minecraft_path);

        //create <name>\data\minecraft\functions
        let minecraft_func_path = format!(r"{}\functions", minecraft_path);
        create_dir_mine(&minecraft_func_path);

        if let Some(t) = b.load {
            //create <name>\data\minecraft\functions\load.mcfunction
            let minecraft_func_load_path = format!(r"{}\load.mcfunction",minecraft_func_path);
            let data = create_function()
        }
    }






}

//todo: エラーメッセージ
fn create_dir_mine(path: &String) {
    if let Err(e) = create_dir(path) {
        panic!("error: {}", e)
    }
}

fn create_file_mine(path: &String, data: &String) {
    let mut file = match File::create(path) {
        Err(e) => panic!("error: {}", e),
        Ok(d) => d,
    };

    if let Err(e) = file.write_all(data.as_bytes()) {
        panic!("error: {}", e)
    }
}
