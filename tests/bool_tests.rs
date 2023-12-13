use i_overlay::bool::overlay_rule::OverlayRule;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fill_rule() {
        let rule = OverlayRule::Subject;
        
        assert_eq!(rule, OverlayRule::Subject);
    }
}