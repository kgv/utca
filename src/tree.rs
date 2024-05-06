use serde::{Deserialize, Serialize};

/// Tree
pub type Tree<M, D> = Branch<M, D>;

/// Node
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Node<M, D> {
    Branch(Branch<M, D>),
    Leaf(Leaf<D>),
}

impl<M, D> From<D> for Node<M, D> {
    fn from(value: D) -> Self {
        Self::Leaf(value.into())
    }
}

impl<'a, M, D> IntoIterator for &'a Node<M, D> {
    type Item = Hierarchized<Item<&'a M, &'a D>>;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::from(self)
    }
}

/// Branch
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Branch<M, D> {
    pub meta: M,
    pub children: Vec<Node<M, D>>,
}

impl<M, D> Branch<M, D> {
    pub fn leaves(&self) -> impl Iterator<Item = Leaf<&D>> {
        self.children
            .iter()
            .flatten()
            .filter_map(|Hierarchized(_, item)| match item {
                Item::Data(data) => Some(Leaf { data }),
                Item::Meta(_) => None,
            })
    }

    pub fn hierarchy(&self) -> impl Iterator<Item = Hierarchized<Item<&M, &D>>> {
        Iter::from_iter(&self.children)
    }

    pub fn items(&self) -> impl Iterator<Item = Item<&M, &D>> {
        Items::from_iter(&self.children)
    }
}

impl<M, D> From<(M, Vec<Node<M, D>>)> for Branch<M, D> {
    fn from((meta, children): (M, Vec<Node<M, D>>)) -> Self {
        Self { meta, children }
    }
}

/// Leaf
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Leaf<D> {
    pub data: D,
}

impl<D> From<D> for Leaf<D> {
    fn from(value: D) -> Self {
        Self { data: value }
    }
}

/// Item
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Item<M, D> {
    Meta(M),
    Data(D),
}

/// Items iterator
struct Items<'a, M, D> {
    stack: Vec<&'a Node<M, D>>,
}

impl<'a, M, D> From<&'a Node<M, D>> for Items<'a, M, D> {
    fn from(value: &'a Node<M, D>) -> Self {
        Self { stack: vec![value] }
    }
}

impl<'a, M, D> FromIterator<&'a Node<M, D>> for Items<'a, M, D> {
    fn from_iter<T: IntoIterator<Item = &'a Node<M, D>>>(iter: T) -> Self {
        Self {
            stack: Vec::from_iter(iter),
        }
    }
}

impl<'a, M, D> Iterator for Items<'a, M, D> {
    type Item = Item<&'a M, &'a D>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        let item = match node {
            Node::Branch(Branch { meta, children }) => {
                for child in children.iter().rev() {
                    self.stack.push(child);
                }
                Item::Meta(meta)
            }
            Node::Leaf(Leaf { data }) => Item::Data(data),
        };
        Some(item)
    }
}

// struct HierarchyItems<I> {
//     hierarchy: Hierarchy,
//     items: I,
// }

// impl<'a, I: Iterator<Item = Item<&'a M, &'a D>>, M: 'a, D: 'a> Iterator for HierarchyItems<I> {
//     type Item = Hierarchized<Item<&'a M, &'a D>>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let node = self.items.next()?;
//         match node {
//             Item::Meta(meta) => Some(Item::Meta(Hierarchized(self.hierarchy.child(index), meta))),
//             Item::Data(data) => Some(Item::Data(data)),
//         }
//     }
// }

/// Reference iterator
struct Iter<'a, M, D> {
    stack: Vec<Hierarchized<&'a Node<M, D>>>,
}

impl<'a, M, D> From<&'a Node<M, D>> for Iter<'a, M, D> {
    fn from(value: &'a Node<M, D>) -> Self {
        Self {
            stack: vec![Hierarchized(Hierarchy::default(), value)],
        }
    }
}

impl<'a, M, D> FromIterator<&'a Node<M, D>> for Iter<'a, M, D> {
    fn from_iter<T: IntoIterator<Item = &'a Node<M, D>>>(iter: T) -> Self {
        let mut stack: Vec<_> = iter.into_iter().enumerate().map(root).collect();
        stack.reverse();
        Self { stack }
    }
}

impl<'a, M, D> Iterator for Iter<'a, M, D> {
    type Item = Hierarchized<Item<&'a M, &'a D>>;

    fn next(&mut self) -> Option<Self::Item> {
        let Hierarchized(hierarchy, node) = self.stack.pop()?;
        let item = match node {
            Node::Branch(Branch { meta, children }) => {
                for (index, child) in children.iter().enumerate().rev() {
                    self.stack.push(Hierarchized(hierarchy.child(index), child));
                }
                Item::Meta(meta)
            }
            Node::Leaf(Leaf { data }) => Item::Data(data),
        };
        Some(Hierarchized(hierarchy, item))
    }
}

/// Hierarchized
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Hierarchized<T>(pub Hierarchy, pub T);

/// Hierarchy
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Hierarchy {
    pub level: usize,
    pub index: usize,
}

impl Hierarchy {
    fn root(index: usize) -> Self {
        Self { level: 0, index }
    }

    fn child(&self, index: usize) -> Self {
        Self {
            level: self.level + 1,
            index,
        }
    }
}

fn root<T>((index, t): (usize, T)) -> Hierarchized<T> {
    Hierarchized(Hierarchy::root(index), t)
}
