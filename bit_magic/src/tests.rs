use super::*;

#[test]
fn test_count_set_bits() {
    assert_eq!(count_set_bits(31), 5);
    assert_eq!(count_set_bits(32), 1);
}

#[test]
fn test_get_2_odd_repeats() {
    println!("{:?}", get_2_odd_repeats(vec![22, 33, 22, 33, 4, 5, 5, 33]));

    assert_eq!(
        get_2_odd_repeats(vec![2, 5, 3, 3, 4, 5, 4, 5, 2, 2, 1, 1]),
        [2, 5]
    )
}

#[test]
fn test_count_total_setbits_till_n() {
    assert_eq!(28, count_total_setbits_till_n(14));
    assert_eq!(32, count_total_setbits_till_n(15));
    assert_eq!(2, count_total_setbits_till_n(2));
}

#[test]
fn test_test_power_of_2() {
    assert_eq!(test_power_of_2(2_u128.pow(100)), true);
    assert_eq!(test_power_of_2(2_u128.pow(099)), true);
    assert_eq!(test_power_of_2(2_u128.pow(098)), true);
    assert_eq!(test_power_of_2(2_u128.pow(096)), true);
}

#[test]
fn test_sparse_bits() {
    assert!(sparse_bits(5));
}
