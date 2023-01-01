/// コマンドを表す列挙型
pub enum Commands {
    Say(String),
}

/// data/functionの中身を表す
pub struct Function {
    load: Option<Vec<Commands>>,
    tick: Option<Vec<Commands>>,
    other: Vec<File>,
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
    name: String,
    data: Vec<Commands>,
}

impl FuncFile {
    /// 新しくmcfunctionを作り、データを返す
    pub fn new(name: String, data: Vec<Commands>) -> FuncFile {
        FuncFile { name, data }
    }
}

pub struct Directory {
    name: String,
    data: Vec<FuncFile>,
}

impl Directory {
    /// 新しくmcfunctionを作り、データを返す
    pub fn new(name: String, data: Vec<FuncFile>) -> Directory {
        Directory { name, data }
    }
}
