mod tester;

use tester::Tester;

#[test]
fn test_pretty_printing() {
    let tester = Tester::new();
    tester.test("ugly.json", "pretty.json");
}
