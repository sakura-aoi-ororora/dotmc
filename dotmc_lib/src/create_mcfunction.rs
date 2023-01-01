use crate::func::Commands::Say;
use crate::func::FuncFile;

pub fn create_function(data: FuncFile) -> (String, String) {
    let name = data.name;
    let mut raw = String::new();
    let at = r##"## <auto-generated by="dotmc">
## このファンクションは、"dotmc" によって生成されたものです。変更しないでください。
## This function is generated by "dotmc". Do not change.
## </auto-generated>"##;

    raw = at.to_string();

    //todo: コマンドの追加
    for command in data.data {
        match command {
            Say(s) => {
                let com = format!("say {}", s);
                raw = format!("{}\n{}", raw, com);
            }
            _ => panic!("コマンドが登録されていません"),
        }
    }

    (name, raw)
}

#[test]
fn say_test() {
    let func = FuncFile::new(
        "test".to_string(),
        vec![Say("hello".to_string()), Say("hello2".to_string())],
    );
    let (name, data) = create_function(func);

    println!("name:{},data:{}", name, data)
}

/*
## <auto-generated by="dotmc">
## このファンクションは、"dotmc" によって生成されたものです。変更しないでください。
## This function is generated by "dotmc". Do not change.
## </auto-generated>t
*/