pub mod graph {
    use std::collections::HashMap;

    use crate::graph::graph_items::{
        edge::Edge,
        node::{ Node},
    };

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }
            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Self {
                        from: a.to_string(),
                        to: b.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs.iter() {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    return self;
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| s.as_str())
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;
            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs.iter() {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    return self;
                }
                      pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| s.as_str())
                }
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend_from_slice(nodes);
            return self;
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend_from_slice(edges);
            return self;
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (k, v) in attrs.iter() {
                self.attrs.insert(k.to_string(), v.to_string());
            }
            return self;
        }
            pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name == name)
        }
    }
}
