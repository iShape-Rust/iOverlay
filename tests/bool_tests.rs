use i_overlay::bool::fill_rule::FillRule;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fill_rule() {
        let rule = FillRule::Subject;
        
        assert_eq!(rule, FillRule::Subject);
    }
}