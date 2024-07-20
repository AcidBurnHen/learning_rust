// Link list

#[derive(Debug)]
struct LinkList {
    head: pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: pointer,
}

type pointer = Option<Box<Node>>;

fn main() {
    let list = Node {
        element: 1,
        next: None,
    };

    let list2 = Node {
        element: 2,
        next: Some(Box::new(Node {
            element: 3,
            next: None,
        })),
    };

    // let list3 = LinkList {
    //     head: Some(Node {
    //         element: 1,
    //         next: Some(Box::new(Node {
    //             element: 2,
    //             next: Some(Box::new(Node {
    //                 element: 3,
    //                 next: None,
    //             })),
    //         })),
    //     }),
    // };

    // let list3 = LinkList { head: None };

    let list4 = LinkList {
        head: Some(Box::new(Node {
            element: 1,
            next: Some(Box::new(Node {
                element: 2,
                next: Some(Box::new(Node {
                    element: 3,
                    next: None,
                })),
            })),
        })),
    };

    println!("{:?}", &list4.head.unwrap().element);
}
