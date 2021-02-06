use std::mem;

/// # chapter 1
/// In this chapter, we have implemented an integer stack without any lifetime specifier.
/// Because we don't hold any reference in the structs. We use smart pointers instead.
/// And we need to care about ownership issues very much.
/// ## match
/// if we don't use ref, there will be an ownership changing in the match expression
#[derive(Debug)]
pub struct List {
    head: Link,
}

// a link is just something like pointer in C/C++
#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

// simply do an iteration
impl Drop for List {
    // this function will be called once the list is deconstructed.
    fn drop(&mut self) {
        // a very common pattern : replace to modify. If we don't do that,
        // the rustc will tell us that we can't move
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            println!("dropped {}", boxed_node.elem)
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push_front(&mut self, elem: i32) {
        // insert before
        let new_node = Box::new(Node {
            elem,
            // use replace to avoid dangling pointer
            next: mem::replace(&mut self.head, Link::Empty),
        });

        // put the right node in it.
        self.head = Link::More(new_node);
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            // we've got the ownership of the previous head here.
            // and it will be dropped automatically after this match scope
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

mod test {
    use super::List;

    #[test]
    fn basic() {
        let mut l = List::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(2);
        l.push_front(2);
        l.push_front(2);
        l.push_front(2);
        l.push_front(2);
        l.push_front(2);
        let a = l.pop_front().expect("opps");
        assert_eq!(a, 2);
    }
}
