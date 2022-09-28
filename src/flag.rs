use clap::{Args, Parser};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Flag {
    // 目录和 zip/jar 文件的类搜索路径
    #[clap(long, value_parser, value_name = "PATH")]
    pub classpath: Option<String>,
    #[clap(long, value_parser)]
    pub jar: String,
    // // Turn debugging information on
    // #[clap(short, action = clap::ArgAction::SetTrue)]
    // pub debug: Option<bool>,
    #[clap(value_parser)]
    pub args: Vec<String>,
}
