extern crate xflow;
use xflow::*;

#[cfg(test)]
#[test]
fn test_store_has() {
    let mut xfstate = XFState::new();
    let xvar = XFlowVariable {
        name: "number1".to_string(),
        vtype: "number1".to_string(),
        value: "number1".to_string(),
    };

    assert_eq!(xfstate.len(), 0);
    assert_eq!(xfstate.is_empty(), true);

    xfstate.add(&xvar);

    assert_eq!(xfstate.len(), 1);
    assert_eq!(xfstate.is_empty(), false);

    assert_eq!(xfstate.has("number1"), true);

    match xfstate.get("number1") {
        Some(res) => assert_eq!(res.name, "number1"),
        None => println!("Error, number1 not found in xfstate"),
    }

}
