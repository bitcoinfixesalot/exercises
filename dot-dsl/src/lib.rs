pub mod graph {
    use crate::converter;
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = converter::attrs_to_hashmap(attrs);
            self
        }

        pub fn get_node(&self, v: &str) -> Option<&Node> {
            self.nodes.iter().find(|a| a.v == v)
        }

        pub fn get_attr(&self, k: &str) -> Option<&str> {
            self.attrs.get(k).and_then(|value| Some(value.as_str()))
        }
    }

    pub mod graph_items {
        pub mod edge {
            use crate::converter;
            use std::collections::HashMap;

            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct Edge {
                pub x: String,
                pub y: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(x: &str, y: &str) -> Self {
                    Edge {
                        x: x.to_string(),
                        y: y.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = converter::attrs_to_hashmap(attrs);
                    self
                }

                pub fn get_attr(&self, k: &str) -> Option<&str> {
                    self.attrs.get(k).and_then(|value| Some(value.as_str()))
                }
            }
        }
        pub mod node {
            use crate::converter;
            use std::collections::HashMap;

            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct Node {
                pub v: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(v: &str) -> Self {
                    Node {
                        v: v.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = converter::attrs_to_hashmap(attrs);
                    self
                }

                pub fn get_attr(&self, k: &str) -> Option<&str> {
                    self.attrs.get(k).and_then(|value| Some(value.as_str()))
                }
            }
        }
    }
}

mod converter {
    use std::collections::HashMap;

    pub fn attrs_to_hashmap<'a, 'b, 'c>(
        attrs: &'a [(&'b str, &'c str)],
    ) -> HashMap<String, String> {
        attrs
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect()
    }
}
