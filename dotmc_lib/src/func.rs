//todo: 進捗、ルートテーブル、その他諸々の追加
pub struct Datapack {
    pub function: Option<Box<Function>>,
}

impl Datapack {
    pub fn new(func: Option<Box<Function>>) -> Datapack {
        Datapack { function: func }
    }
}

//todo: コマンドの追加
/// コマンドを表す列挙型
pub enum Commands {
    Say(String),
}

/// data/functionの中身を表す
pub struct Function {
    pub load: Option<Vec<Commands>>,
    pub tick: Option<Vec<Commands>>,
    pub other: Vec<File>,
}

impl Function {
    /// 新しくfunctionを作り、データを返す
    pub fn new(
        load: Option<Vec<Commands>>,
        tick: Option<Vec<Commands>>,
        other: Vec<File>,
    ) -> Box<Function> {
        Box::new(Function { load, tick, other })
    }
}

pub enum File {
    Function(FuncFile),
    Directory(Directory),
}

/// 普通のmcfunctionを表す
pub struct FuncFile {
    pub name: String,
    pub data: Vec<Commands>,
}

impl FuncFile {
    /// 新しくmcfunctionを作り、データを返す
    pub fn new(name: String, data: Vec<Commands>) -> FuncFile {
        FuncFile { name, data }
    }
}

pub struct Directory {
    pub name: String,
    pub data: Vec<File>,
}

impl Directory {
    /// 新しくDirectoryを作り、データを返す
    pub fn new(name: String, data: Vec<File>) -> Directory {
        Directory { name, data }
    }
}
