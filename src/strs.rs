/// 字符串操作类 <br>
/// since: 1.0
pub struct Strs {}

impl Strs {
    /// 从指定下标寻找目标字符串 <br>
    /// Params: <br>
    /// &emsp; s - 要操作的字符串切片 <br>
    /// &emsp; from_idx - 开始查找的下标 <br>
    /// &emsp; target - 查找的字符串切片 <br>
    /// Returns: 目标字符串下标
    pub fn index_of(s: &str, from_idx: usize, target: &str) -> Option<usize> {
        // 利用字符串切片查找目标字符串
        match &s[from_idx..].find(target) {
            None => None,
            Some(res) => Some(*res + from_idx)
        }
    }
}
