use crate::create_mcfunction::create_function_vec;
use crate::func::{Commands, Datapack, FuncFile, Function};
use std::fs::{create_dir, read_to_string, remove_dir_all, File};
use std::io::Write;
use std::path::Path;

pub fn create_datapack(datapack: Datapack, path: &str, name: &str) {
    let path = String::from(path);
    let name = String::from(name);

    //create datapack(<name>)
    let path = format!(r"{}\{}", path, name);

    {
        let test_path = Path::new(&path);
        let re = Path::try_exists(test_path);
        let unw = match re {
            Ok(b) => b,
            Err(e) => panic!("error: {}", e),
        };

        if unw {
            let re = remove_dir_all(test_path);
            if let Err(e) = re {
                panic!("{}", e);
            }
        }
    }

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

    //create <name>\data
    let data_path = path + r"\data"; // pathの所有権を消す
    create_dir_mine(&data_path);

    //create <name>\data\<name>
    let name_path = format!(r"{}\{}", data_path, name);
    create_dir_mine(&name_path);

    //todo: 進捗、ルートテーブル、その他諸々の追加
    //todo: 要素の削除系

    //functionの編集
    if let Some(b) = datapack.function {
        let is_load = b.load.is_some();
        let is_tick = b.tick.is_some();

        //create <name>\data\<name>\functions
        let name_function_path = format!(r"{}\functions", name_path);
        create_dir_mine(&name_function_path);

        if is_load || is_tick {
            let load_com = if let Some(t) = b.load {
                t
            } else {
                panic!("バグ")
            };

            let tick_com = if let Some(t) = b.tick {
                t
            } else {
                panic!("バグ")
            };

            //minecraft側の編集
            {
                // create <name>\data\minecraft
                let minecraft_path = format!(r"{}\minecraft", data_path);
                create_dir_mine(&minecraft_path);

                // create <name>\data\minecraft\tags
                let minecraft_tags_path = format!(r"{}\tags", minecraft_path);
                create_dir_mine(&minecraft_tags_path);

                // create <name>\data\minecraft\tags\functions
                let minecraft_tags_functions_path = format!(r"{}\functions", minecraft_tags_path);
                create_dir_mine(&minecraft_tags_functions_path);

                if is_load {
                    // create <name>\data\minecraft\tags\functions\load.json
                    let minecraft_tags_function_load_path =
                        format!(r"{}\load.json", minecraft_tags_functions_path);
                    let json = format!(r###"{{"values":["{}:load"]}}"###, name);
                    create_file_mine(&minecraft_tags_function_load_path, &json);
                }

                if is_tick {
                    // create <name>\data\minecraft\tags\functions\tick.json
                    let minecraft_tags_function_tick_path =
                        format!(r"{}\tick.json", minecraft_tags_functions_path);
                    let json = format!(r###"{{"values":["{}:tick"]}}"###, name);
                    create_file_mine(&minecraft_tags_function_tick_path, &json);
                }
            }

            // <name>側の編集
            {
                if is_load {
                    //create <name>\data\<name>\functions\load.mcfunction
                    let name_func_load_path = format!(r"{}\load.mcfunction", name_function_path);
                    let com = create_function_vec(load_com);
                    create_file_mine(&name_func_load_path, &com);
                }

                if is_tick {
                    //create <name>\data\<name>\functions\tick.mcfunction
                    let name_func_tick_path = format!(r"{}\tick.mcfunction", name_function_path);
                    let com = create_function_vec(tick_com);
                    create_file_mine(&name_func_tick_path, &com);
                }
            }
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

#[test]
fn test() {
    let load = vec![
        Commands::Say("hello".to_string()),
        Commands::Say("load".to_string()),
        Commands::Say("hello".to_string()),
    ];
    let tick = vec![
        Commands::Say("hello".to_string()),
        Commands::Say("tick".to_string()),
    ];
    let other = FuncFile::new(
        "test".to_string(),
        vec![
            Commands::Say("hello".to_string()),
            Commands::Say("other".to_string()),
        ],
    );
    let func = Function::new(
        Some(load),
        Some(tick),
        vec![crate::func::File::Function(other)],
    );
    let pack = Datapack::new(Some(func));
    let path = r""; //変更する
    create_datapack(pack, path, "test_gen")
}

#[test]
fn test2() {
    let path = Path::new(r""); // 変更する
    let re = Path::try_exists(path);
    match re {
        Ok(b) => println!("{}", b),
        Err(e) => println!("error: {}", e),
    }
}
