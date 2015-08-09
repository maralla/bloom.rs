extern crate bloom;

#[test]
fn test_bloom() {
    let mut bf = bloom::Bloom::new(20, 0.01).unwrap();
    let keys = ["foo", "bar", "foosdfsdfs", "fossdfsdfo",
                "foasdfasdfasdfasdfo", "foasdfasdfasdasdfasdfasdfasdfasdfo"];
    let faux = ["goo", "gar", "gaz"];

    for k in keys.iter() {
        bf.add(k);
    }

    for k in keys.iter() {
        assert!(bf.check(k));
    }

    for k in faux.iter() {
        assert!(!bf.check(k));
    }
}
