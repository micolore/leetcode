///
/// 回文
///
pub fn palindrome_str_low() {
    println!("wow this palindrome test!");

    let s = String::from("daaad");
    let mut _index = 0;
    let mut _iner_index = 0;
    let mut _len = s.len();
    println!("str len:{}", &_len);

    for _index in 0..s.len() {
        let mut _index_str = &s[_index.._index + 1];
        let mut _len_str = &s[&_len - _index - 1.._len - _index];
        //println!("_index_str:{},_len_str:{}", _index_str, _len_str);
        if _index_str != _len_str {
            println!("str is not palindrome");
            return;
        }
    }
    println!("str is palindrome");
}

pub fn palindrome_str_high() {

    
}
