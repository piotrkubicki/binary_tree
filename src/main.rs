const MAX_LEVEL: usize = 2;

enum Leaf<T> {
    Item(Box<T>),
    Empty,
}

struct Node<T> {
    value: T,
    left: Leaf<Node<T>>,
    right: Leaf<Node<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: Leaf::Empty,
            right: Leaf::Empty,
        }
    }

    fn _add_node(&mut self, node: Box<Node<T>>, iteration: usize) -> Result<(), String> {
        if iteration > MAX_LEVEL {
            return Err("Max iteration achieved".to_string());
        }

        if matches!(self.left, Leaf::Empty) {
            self.left = Leaf::Item(node);
        } else if matches!(self.right, Leaf::Empty) {
            self.right = Leaf::Item(node);
        } else {
            if let Leaf::Item(n) = &mut self.left {
                n._add_node(node, iteration + 1)?;
            }
        }

        Ok(())
    }

    fn add_node(&mut self, node: Box<Node<T>>) -> Result<(), String> {
        self._add_node(node, 1)
    }
}

impl<T> Node<T>
    where T: std::ops::Add + std::ops::AddAssign + Clone {
    fn sum(&self) -> T {
        let mut total = self.value.clone();
        if let Leaf::Item(item) = &self.left {
            total += item.sum();
        }
        if let Leaf::Item(item) = &self.right {
            total += item.sum();
        }

        total
    }
}

fn main() -> Result<(), String> {
    let mut base = Node::new(2);
    let l1 = Node::new(5);
    let r1 = Node::new(4);
    let l2 = Node::new(1);

    base.add_node(Box::new(l1))?;
    base.add_node(Box::new(r1))?;
    base.add_node(Box::new(l2))?;

    let total = base.sum();
    println!("Total: {}", total);

    if let Leaf::Item(l1) = &base.left {
        let total = l1.sum();
        println!("Total: {}", total);
    }
    let total = base.sum();
    println!("Total: {}", total);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_node_for_left_leaf_returns_ok_if_empty() {
        let mut n1 = Node::new(1);
        let n2 = Node::new(2);

        assert_eq!(Ok(()), n1.add_node(Box::new(n2)));
        if let Leaf::Item(left) = &n1.left {
            assert_eq!(2, left.value);
        }

        let n3 = Node::new(3);
        assert_eq!(Ok(()), n1.add_node(Box::new(n3)));
        if let Leaf::Item(right) = &n1.right {
            assert_eq!(3, right.value);
        }

        let n4 = Node::new(4);
        assert_eq!(Ok(()), n1.add_node(Box::new(n4)));
        if let Leaf::Item(left) = &n1.left {
            if let Leaf::Item(left) = &left.left {
                assert_eq!(4, left.value);
            }
        }

        let n5 = Node::new(5);
        assert_eq!(Ok(()), n1.add_node(Box::new(n5)));
        if let Leaf::Item(right) = &n1.right {
            if let Leaf::Item(right) = &right.right {
                assert_eq!(5, right.value);
            }
        }

        let n6 = Node::new(6);
        assert_eq!(Err("Max iteration achieved".to_string()), n1.add_node(Box::new(n6)))
    }

    #[test]
    fn node_sum_returns_correct_value() -> Result<(), String> {
        let mut base = Node::new(2);
        let l1 = Node::new(5);
        let r1 = Node::new(4);
        let l2 = Node::new(1);

        base.add_node(Box::new(l1))?;
        base.add_node(Box::new(r1))?;
        base.add_node(Box::new(l2))?;

        assert_eq!(12, base.sum());

        if let Leaf::Item(l1) = &base.left {
            assert_eq!(6, l1.sum());
        }

        Ok(())
    }
}
