#[cfg(test)]
pub mod tests {
    use super::super::guess_num::*;
    use super::super::rectangle::*;

    #[test]
    fn passes_fit() {
        let r1 = Rectanlge::new(5, 5);
        let r2 = Rectanlge::new(4, 4);
        assert!(r1.fits(&r2));
        assert!(!r2.fits(&r1));
    }

    // #[test]
    // fn passes_area() {
    //     assert_eq!(Rectanlge::new(5, 5).area(), 25);
    //     assert_ne!(Rectanlge::new(4, 5).area(), 25);
    // }

    #[test]
    #[ignore]
    // Custom faliure message
    fn fails_fit() {
        let r1 = Rectanlge::new(5, 5);
        let r2 = Rectanlge::new(4, 4);
        assert!(
            r2.fits(&r1),
            "The rectangle is smaller than the argument rectangle."
        );
    }

    #[test]
    #[ignore]
    #[should_panic(expected = "Guess is greater than valid guess")]
    // assert function panic
    fn function_panics() {
        Guess::new(9);
    }
}

#[cfg(test)]
mod tests2 {

    use crate::rustlings::*;

    mod tests {
        use super::*;

        #[test]
        fn generates_nametag_text_for_a_nonempty_name() {
            assert_eq!(
                generate_nametag_text("Beyoncé".into()),
                Some("Hi! My name is Beyoncé".into())
            );
        }

        #[test]
        fn explains_why_generating_nametag_text_fails() {
            assert_ne!(
                generate_nametag_text("".into()),
                // Don't change this line
                Some("`name` was empty; it must be nonempty.".into())
            );
        }
    }
}
