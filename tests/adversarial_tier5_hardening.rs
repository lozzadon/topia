use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use topia::{Node, TopiaApp};

/// 1. Deep nesting stress: 1,000 nested containers
#[test]
fn test_tier5_deeply_nested_layout_and_traversal() {
    let mut root = Node::text("Leaf at depth 1000");
    for i in 0..1000 {
        if i % 2 == 0 {
            root = Node::vstack(vec![root]);
        } else {
            root = Node::hstack(vec![root]);
        }
    }

    // Traverse down 1,000 levels
    let mut current = &root;
    let mut depth = 0;
    while current.child_count() > 0 {
        depth += 1;
        current = &current.children()[0];
    }
    assert_eq!(depth, 1000);
    assert_eq!(current.as_text(), Some("Leaf at depth 1000"));
}

/// 2. Extreme and abnormal floating point spacings
#[test]
fn test_tier5_extreme_floating_point_spacing() {
    let spacings = [
        0.0f32,
        -0.0f32,
        -100.0f32,
        f32::MIN,
        f32::MAX,
        f32::MIN_POSITIVE,
        f32::EPSILON,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
    ];

    for &s in &spacings {
        let v = Node::vstack_with_spacing(vec![Node::text("A"), Node::text("B")], s);
        let h = Node::hstack_with_spacing(vec![Node::text("C"), Node::text("D")], s);

        if let Node::VStack { spacing, .. } = v {
            if s.is_nan() {
                assert!(spacing.unwrap().is_nan());
            } else {
                assert_eq!(spacing, Some(s));
            }
        }
        if let Node::HStack { spacing, .. } = h {
            if s.is_nan() {
                assert!(spacing.unwrap().is_nan());
            } else {
                assert_eq!(spacing, Some(s));
            }
        }
    }
}

/// 3. Concurrent multi-threaded callback executions
#[test]
fn test_tier5_concurrent_thread_callback_execution() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    // Spawn 10 threads, each firing 500 clicks
    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut btn = Node::button("Thread Click", move || {
                c.fetch_add(1, Ordering::SeqCst);
            });
            for _ in 0..500 {
                assert!(btn.fire_click());
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().expect("Thread panicked");
    }

    assert_eq!(counter.load(Ordering::SeqCst), 5000);
}

/// 4. Memory drop hierarchy & exact cleanup invariants
#[test]
fn test_tier5_memory_drop_hierarchy_cleanliness() {
    let drop_counter = Arc::new(AtomicUsize::new(0));

    struct DropGuard {
        counter: Arc<AtomicUsize>,
    }
    impl Drop for DropGuard {
        fn drop(&mut self) {
            self.counter.fetch_add(1, Ordering::SeqCst);
        }
    }

    const NODE_COUNT: usize = 2000;
    {
        let mut children = Vec::with_capacity(NODE_COUNT);
        for _ in 0..NODE_COUNT {
            let guard = DropGuard {
                counter: Arc::clone(&drop_counter),
            };
            children.push(Node::button("Click", move || {
                let _ = &guard;
            }));
        }
        let tree = Node::vstack(children);
        assert_eq!(tree.child_count(), NODE_COUNT);
        assert_eq!(drop_counter.load(Ordering::SeqCst), 0);
        drop(tree);
    }

    assert_eq!(drop_counter.load(Ordering::SeqCst), NODE_COUNT);
}

/// 5. Rapid dynamic reconfiguration across 1,000 frames
#[test]
fn test_tier5_rapid_dynamic_reconfiguration_simulation() {
    let state = Rc::new(RefCell::new(0));
    let state_for_app = state.clone();

    let mut app = TopiaApp::new(move || {
        let val = *state_for_app.borrow();
        match val % 5 {
            0 => Node::empty(),
            1 => Node::text(format!("Text {}", val)),
            2 => Node::button(format!("Btn {}", val), || {}),
            3 => Node::vstack(vec![Node::text("V1"), Node::text("V2")]),
            4 => Node::hstack(vec![Node::button("H1", || {}), Node::button("H2", || {})]),
            _ => unreachable!(),
        }
    });

    for frame in 0..1000 {
        *state.borrow_mut() = frame;
        let node = (app.view_builder)();
        match frame % 5 {
            0 => assert!(node.is_empty()),
            1 => assert_eq!(node.as_text(), Some(format!("Text {}", frame).as_str())),
            2 => assert_eq!(node.as_button_label(), Some(format!("Btn {}", frame).as_str())),
            3 => {
                assert_eq!(node.child_count(), 2);
                assert_eq!(node.children()[0].as_text(), Some("V1"));
            }
            4 => {
                assert_eq!(node.child_count(), 2);
                assert_eq!(node.children()[1].as_button_label(), Some("H2"));
            }
            _ => unreachable!(),
        }
    }
}

/// 6. Large payload text handling (100,000 characters)
#[test]
fn test_tier5_large_payload_text_node() {
    let large_text = "🌍 Topia Native Declarative UI ⚡ ".repeat(3000);
    let expected_len = large_text.len();

    let text_node = Node::text(large_text);
    assert_eq!(text_node.as_text().map(|s| s.len()), Some(expected_len));
}
