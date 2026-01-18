use std::rc::Rc;

pub fn chapter_work() {
    tuple_on_heap();
    struct_ownership();
    moves_example();
    copyable_struct();
    shared_ownership();
}


fn tuple_on_heap() {
    let point = Box::new((0.625, 0.5));
    let label = format!("{:?}", point);
    assert_eq!(label, "(0.625, 0.5)");
}


fn struct_ownership() {
    struct Person {
        name: String,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(
        Person {
           name: "Palestrina".to_string(),
            birth: 1525,
        }
    );
    composers.push(
        Person {
            name: "Dowland".to_string(),
            birth: 1563,
        }
    );
    composers.push(
        Person {
            name: "Lully".to_string(),
            birth: 1632,
        }
    );

    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth)
    }
}

fn assignment_example() {
    {
        let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
        let t = s;
        // let u = s; This would fail since we moved s into t
    }

    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s.clone();
    let u = s.clone();
}

fn moves_example() {
    {
        let mut v = Vec::new();
        for i in 101 .. 106 {
            v.push(i.to_string());
        }

        // let third = v[2]; ERROR: Cannot move out of index of Vec
        // let fifth = v[4]; Similar error
    }

    // Build a vector of the strings "101", "102", ... "105"
    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }

    // 1. Pop a value off the end of the vector:
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    // 2. Move a value out of a given index in the vector
    // and move the last element into its spot:
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // 3. Swap in another value for the one we're taking out:
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    // Let's see what's left of our vector
    assert_eq!(v, vec!["101", "104", "substitute"]);


    struct Person {
        name: Option<String>,
        hometown: Option<String>,
        birth: i32,
    }

    let mut composers = Vec::new();

    composers.push(
        Person {
            name: Some("Palestrina".to_string()),
            hometown: Some("Kansas".to_string()),
            birth: 1525,
        }
    );

    // Long format
    let first_name = std::mem::replace(&mut composers[0].name, None);
    // Shorthand
    let hometown = composers[0].hometown.take();

    assert_eq!(first_name, Some("Palestrina".to_string()));
    assert_eq!(hometown, Some("Kansas".to_string()));
    assert_eq!(composers[0].name, None);
    assert_eq!(composers[0].hometown, None);
}


fn copyable_struct() {
    struct UncopyableLabel {
        number: u32,
    }

    fn uncopyable_print(l: UncopyableLabel) {
        println!("STAMP: {}", l.number);
    }

    let l = UncopyableLabel {
        number: 3,
    };
    uncopyable_print(l);
    // println!("My label number is: {}", l.number);
    // Error! Even though the only field is a copyable u32,
    // it doesn't allow us to implicitly do a bit-by-bit deep copy
    // unless explicitly told to.
    #[derive(Copy, Clone)]
    struct CopyableLabel {
        number: u32,
    }

    fn copyable_print(l: CopyableLabel) {
        println!("STAMP: {}", l.number);
    }

    let l = CopyableLabel {
        number: 5
    };

    // Now we can reuse the variable,
    // and a deep bit-by-bit copy will be made w/o using .clone
    copyable_print(l);


    // Something to remember is that this only works with types that with the Copy trait
    // #[derive(Copy, Clone)]
    // struct StringLabel { name: String } Won't compile!!!

}


fn shared_ownership() {
    use std::rc::Rc;

    let s = Rc::new("shirataki".to_string());
    // Cloning an Rc<T> value doesn't copy the T
    // Instead, it creates another pointer to the value and increments the reference count
    let t = s.clone();
    let u = s.clone();

    println!("s: {}\nt: {}\nu: {}", s, t, u);
    assert_eq!(*s, "shirataki");
    assert_eq!(*t, "shirataki");
    assert_eq!(*u, "shirataki");
    assert_eq!(3, Rc::strong_count(&s));
    
}