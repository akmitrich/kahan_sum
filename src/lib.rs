pub mod kahan;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn video_example() {
        let mut ks = kahan::KahanSum::default();
        ks.add(16777216_f32);
        assert_eq!(16777216., ks.get_sum());
        assert_eq!(0., ks.get_err());
        ks.add(1_f32);
        assert_eq!(16777216., ks.get_sum());
        assert_eq!(-1., ks.get_err());
        ks.add(1_f32);
        assert_eq!(16777218., ks.get_sum());
        assert_eq!(0., ks.get_err());
    }
    
    #[test]
    fn test_operator_overloads() {
        let mut ks = kahan::KahanSum::default() + 16777216_f32;
        assert_eq!(16777216., ks.get_sum());
        assert_eq!(0., ks.get_err());
        ks += 1.;
        assert_eq!(16777216., ks.get_sum());
        assert_eq!(-1., ks.get_err());
        ks += 1.;
        assert_eq!(16777218., ks.get_sum());
        assert_eq!(0., ks.get_err());
    }

    #[test]
    fn test_from() {
        let ks = kahan::KahanSum::from(16777216_f32);
        assert_eq!(16777216., ks.get_sum());
        assert_eq!(0., ks.get_err());
        let mut ks = ks + 1.;
        assert_eq!(16777216., ks.get_sum());
        assert_eq!(-1., ks.get_err());
        ks += 1.;
        assert_eq!(16777218., ks.get_sum());
        assert_eq!(0., ks.get_err());
    }

    #[test]
    fn test_sum_iterator() {
        use kahan::KahanSummator;
        let mut ks = kahan::KahanSum::default();
        ks += 102232.3;
        ks += 0.32;
        ks += 13.13;
        let values: Vec<f32> = vec![102232.3, 0.32, 13.13];
        assert_eq!(ks, values.into_iter().kahan_sum());
    }
}
