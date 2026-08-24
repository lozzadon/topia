use std::cell::RefCell;
use std::rc::Rc;
use topia::Node;

#[test]
fn test_direct_button_callback_execution() {
    let counter = Rc::new(RefCell::new(0));

    let mut btn = {
        let c = counter.clone();
        Node::button("Increment", move || {
            *c.borrow_mut() += 1;
        })
    };

    assert_eq!(*counter.borrow(), 0);

    assert!(btn.fire_click());
    assert_eq!(*counter.borrow(), 1);

    assert!(btn.fire_click());
    assert!(btn.fire_click());
    assert_eq!(*counter.borrow(), 3);
}

#[test]
fn test_counter_multi_button_interaction_sequence() {
    let count = Rc::new(RefCell::new(0));

    let mut inc_btn = {
        let c = count.clone();
        Node::button("+", move || *c.borrow_mut() += 1)
    };

    let mut dec_btn = {
        let c = count.clone();
        Node::button("-", move || *c.borrow_mut() -= 1)
    };

    let mut reset_btn = {
        let c = count.clone();
        Node::button("reset", move || *c.borrow_mut() = 0)
    };

    // Simulate clicking: + + + - + (expected = 3)
    inc_btn.fire_click();
    inc_btn.fire_click();
    inc_btn.fire_click();
    dec_btn.fire_click();
    inc_btn.fire_click();
    assert_eq!(*count.borrow(), 3);

    // Reset -> 0
    reset_btn.fire_click();
    assert_eq!(*count.borrow(), 0);

    // Dec -> -1
    dec_btn.fire_click();
    assert_eq!(*count.borrow(), -1);
}

#[test]
fn test_string_and_collection_mutation_in_callbacks() {
    let log = Rc::new(RefCell::new(Vec::<String>::new()));

    let mut add_item = {
        let l = log.clone();
        Node::button("Add Task", move || {
            l.borrow_mut().push("New Task".to_string());
        })
    };

    let mut clear_items = {
        let l = log.clone();
        Node::button("Clear", move || {
            l.borrow_mut().clear();
        })
    };

    add_item.fire_click();
    add_item.fire_click();
    assert_eq!(log.borrow().as_slice(), &["New Task", "New Task"]);

    clear_items.fire_click();
    assert!(log.borrow().is_empty());
}
