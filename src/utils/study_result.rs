use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum StudyResult {
  Success(String),
  Unknown,
  Duplicated,
}

impl fmt::Display for StudyResult {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      StudyResult::Success(_) => write!(f, "成功"),
      StudyResult::Unknown => write!(f, "请求发送了，查询时却没有学习记录，建议自行查询学习状态"),
      StudyResult::Duplicated => write!(f, "重复")
    }
  }
}