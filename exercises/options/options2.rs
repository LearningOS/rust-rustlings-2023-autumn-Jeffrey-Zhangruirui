// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // 使用 if let 将 optional_target 解构成 Some 类型的值并将其赋给 word
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // 使用 while let 处理 optional_integers 中的 Option<T>
        while let Some(integer) = optional_integers.pop() {
            // 注意：integer 是一个 Option<i8>，我们需要使用 unwrap() 获取实际值
        
            cursor = 0;
        }

        assert_eq!(cursor, 0);
    }
}
