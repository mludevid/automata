use std::collections::HashSet;
use std::fmt::Debug;
use std::io::{self, Write};
use std::hash::Hash;

use super::dot::{Family, Node as WriteNode, GraphWriter};
use super::regex::Regex;

/// A node handle.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Node(pub usize);

pub struct Dfa<Alphabet: Eq + Hash> {
    edges: Vec<Vec<(Alphabet, Node)>>,
    finals: Vec<Node>,
}

impl<A: Eq + Hash> Dfa<A> {
    pub fn from_edges<I, V>(edge_iter: I, finals: V) -> Dfa<A>
    where 
        I: IntoIterator<Item=(usize, A, usize)>,
        V: Into<Vec<usize>>, 
        A: Clone + Debug,
    {
        let mut edges = Vec::new();
        let mut check = Vec::new();

        for (from, a, to) in edge_iter.into_iter() {
            edges.resize(from + 1, Vec::new());
            check.resize(from + 1, HashSet::new());
            
            edges[from].push((a.clone(), to));
            check[from].insert(a);
        }

        if let Some(sample) = check.pop() {
            if let Some(err) = check.iter().find(|&s| s != &sample) {
                panic!("Different outgoing edges alphabet: {:?} vs {:?}", &sample, &err);
            }
        }

        unimplemented!()
    }

    pub fn to_regex(self) -> Regex {
        unimplemented!()
    }

    pub fn write_to(&self, output: &mut Write) -> io::Result<()> {
        let mut writer = GraphWriter::new(output, Family::Directed, None)?;

        for (from, edges) in self.edges.iter().enumerate() {
            for (label, to) in edges.iter() {
                writer.segment([from, to.0].iter().cloned(), None)?;
            }
        }

        writer.end_into_inner().1
    }
}

