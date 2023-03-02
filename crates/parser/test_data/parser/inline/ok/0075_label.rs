fn foo() {
    'a: loop {}
    'b: while true {}
    'c: for x in () {}
}
fn label(p: &mut Parser<'_>) {
    assert!(p.nth(0) == T![:]);
    let m = p.start();
    p.bump_any();
    m.complete(p, LABEL);
}
