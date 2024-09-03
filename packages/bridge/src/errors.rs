use std::error::Error;
use std::fmt::{Debug, Display};
use std::fmt::{self};

#[derive(Debug)]
pub struct ErrMsg {
    pub code: u32,
    pub msg: String,
}

// 为 AppError 实现 std::fmt::Display 特征
impl fmt::Display for ErrMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ErrMsg: {{code:{}，msg :{} }}", self.code, self.msg) // user-facing output
    }
}

#[allow(unused)]
pub fn print_err<E>(e: E)
where
    E: Display + Debug + Error,
{
    print!("{}", e);
}
