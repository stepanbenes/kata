struct TreeNode<'a> {
    left: Option<&'a TreeNode<'a>>,
    right: Option<&'a TreeNode<'a>>,
}

// https://www.codewars.com/kata/5426d7a2c2c7784365000783/train/rust
pub fn balanced_parens(_n: u16) -> Vec<String> {
    vec!["".to_owned()]
}
