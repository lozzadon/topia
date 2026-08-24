use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::Arc;
use topia::{Node, TopiaApp};

#[test]
fn test_multiple_callbacks_modifying_complex_shared_state() {
    #[derive(Debug, Default, PartialEq, Eq)]
    struct ComplexAppState {
        counter: i64,
        history: Vec<String>,
        flags: HashMap<String, bool>,
        active_view: String,
    }

    let state = Rc::new(RefCell::new(ComplexAppState {
        active_view: "dashboard".to_string(),
        ..Default::default()
    }));

    // Button 1: Increments counter and appends history
    let mut btn_inc = {
        let s = state.clone();
        Node::button("Increment", move || {
            let mut st = s.borrow_mut();
            st.counter += 1;
            let c = st.counter;
            st.history.push(format!("inc:{}", c));
        })
    };

    // Button 2: Decrements counter and appends history
    let mut btn_dec = {
        let s = state.clone();
        Node::button("Decrement", move || {
            let mut st = s.borrow_mut();
            st.counter -= 1;
            let c = st.counter;
            st.history.push(format!("dec:{}", c));
        })
    };

    // Button 3: Toggle flag and change active view
    let mut btn_toggle = {
        let s = state.clone();
        Node::button("Toggle Setting", move || {
            let mut st = s.borrow_mut();
            let current = *st.flags.get("dark_mode").unwrap_or(&false);
            st.flags.insert("dark_mode".to_string(), !current);
            st.active_view = if !current { "dark_dashboard" } else { "light_dashboard" }.to_string();
        })
    };

    // Button 4: Reset state with custom clear
    let mut btn_reset = {
        let s = state.clone();
        Node::button("Reset", move || {
            let mut st = s.borrow_mut();
            st.counter = 0;
            st.history.clear();
            st.flags.clear();
            st.active_view = "dashboard".to_string();
        })
    };

    // Execute sequence: inc, inc, toggle, dec, inc, toggle
    btn_inc.fire_click();
    btn_inc.fire_click();
    assert_eq!(state.borrow().counter, 2);
    assert_eq!(state.borrow().history, vec!["inc:1", "inc:2"]);

    btn_toggle.fire_click();
    assert_eq!(state.borrow().flags.get("dark_mode"), Some(&true));
    assert_eq!(state.borrow().active_view, "dark_dashboard");

    btn_dec.fire_click();
    assert_eq!(state.borrow().counter, 1);
    assert_eq!(state.borrow().history, vec!["inc:1", "inc:2", "dec:1"]);

    btn_inc.fire_click();
    assert_eq!(state.borrow().counter, 2);

    btn_toggle.fire_click();
    assert_eq!(state.borrow().flags.get("dark_mode"), Some(&false));
    assert_eq!(state.borrow().active_view, "light_dashboard");

    // Reset
    btn_reset.fire_click();
    assert_eq!(state.borrow().counter, 0);
    assert!(state.borrow().history.is_empty());
    assert!(state.borrow().flags.is_empty());
    assert_eq!(state.borrow().active_view, "dashboard");
}

#[test]
fn test_stress_high_frequency_callback_mutations() {
    let atomic_counter = Arc::new(AtomicI64::new(0));
    let mut buttons = Vec::new();

    // Create 100 buttons that modify the same atomic counter with different step values
    for i in 0..100 {
        let cnt = Arc::clone(&atomic_counter);
        let step = (i as i64) - 50; // range -50 .. 49
        let btn = Node::button(format!("Btn {}", i), move || {
            cnt.fetch_add(step, Ordering::SeqCst);
        });
        buttons.push(btn);
    }

    // Fire all buttons 10 times in interleaved patterns
    let mut expected_total: i64 = 0;
    for _ in 0..10 {
        for (i, btn) in buttons.iter_mut().enumerate() {
            let step = (i as i64) - 50;
            assert!(btn.fire_click());
            expected_total += step;
        }
    }

    assert_eq!(atomic_counter.load(Ordering::SeqCst), expected_total);
}

#[test]
fn test_callback_closure_cleanup_and_drop_semantics() {
    let tracker = Arc::new(AtomicUsize::new(0));

    struct DropDetector {
        counter: Arc<AtomicUsize>,
    }
    impl Drop for DropDetector {
        fn drop(&mut self) {
            self.counter.fetch_add(1, Ordering::SeqCst);
        }
    }

    {
        let detector = DropDetector {
            counter: Arc::clone(&tracker),
        };

        // Create a node tree containing a button that captures the detector
        let node = Node::vstack(vec![
            Node::text("Sample"),
            Node::button("Click Me", move || {
                let _ = &detector;
            }),
        ]);

        assert_eq!(tracker.load(Ordering::SeqCst), 0);
        drop(node); // Drop the tree
    }

    // Detector must have been dropped exactly once when Node tree was dropped
    assert_eq!(tracker.load(Ordering::SeqCst), 1);
}

#[test]
fn test_reactive_view_builder_multi_frame_simulation() {
    let state = Rc::new(RefCell::new(0));
    let state_for_builder = state.clone();

    // View builder simulates an immediate-mode UI rendering pass
    let mut topia_app = TopiaApp::new(move || {
        let count = *state_for_builder.borrow();
        Node::vstack(vec![
            Node::text(format!("Counter: {}", count)),
            Node::hstack(vec![
                Node::button("Add", {
                    let s = state_for_builder.clone();
                    move || *s.borrow_mut() += 10
                }),
                Node::button("Sub", {
                    let s = state_for_builder.clone();
                    move || *s.borrow_mut() -= 5
                }),
            ]),
        ])
    });

    // Simulate 500 reactive frames with state modifications
    for frame in 0..500 {
        let mut tree = (topia_app.view_builder)();

        // Check text matches state at beginning of frame
        let expected_val = frame * 5;
        let expected_text = format!("Counter: {}", expected_val);
        assert_eq!(tree.children()[0].as_text(), Some(expected_text.as_str()));

        // Simulate user clicking "Add" (+10) then "Sub" (-5) -> net +5 per frame
        if let Node::VStack { children, .. } = &mut tree {
            if let Node::HStack {
                children: h_children,
                ..
            } = &mut children[1]
            {
                h_children[0].fire_click(); // Add +10
                h_children[1].fire_click(); // Sub -5
            } else {
                panic!("Expected HStack");
            }
        } else {
            panic!("Expected VStack");
        }
    }

    assert_eq!(*state.borrow(), 2500);
}

#[test]
fn test_callback_dynamic_node_generation() {
    let captured_nodes = Rc::new(RefCell::new(Vec::<Node>::new()));

    let mut generator_btn = {
        let store = captured_nodes.clone();
        Node::button("Generate Subtree", move || {
            let new_tree = Node::vstack(vec![
                Node::text("Dynamically Generated Header"),
                Node::button("Inner Action", || {}),
                Node::empty(),
            ]);
            store.borrow_mut().push(new_tree);
        })
    };

    assert_eq!(captured_nodes.borrow().len(), 0);
    generator_btn.fire_click();
    assert_eq!(captured_nodes.borrow().len(), 1);

    generator_btn.fire_click();
    assert_eq!(captured_nodes.borrow().len(), 2);

    let list = captured_nodes.borrow();
    assert_eq!(list[0].child_count(), 3);
    assert_eq!(list[1].child_count(), 3);
}
