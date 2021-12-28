use std::collections::HashMap;

type NodeList = HashMap<String, Node>;

#[derive(Debug)]
pub struct DomainLookup {
    nodes: NodeList,
    minimum_level: usize,
}

#[derive(Debug)]
pub struct Node {
    wildcard: bool,
    nodes: NodeList,
    data: String,
}

impl Node {
    fn new(wildcard: bool, data: &str) -> Self {
        Self {
            wildcard,
            nodes: Default::default(),
            data: data.to_owned(),
        }
    }
}

impl DomainLookup {
    pub fn new() -> DomainLookup {
        DomainLookup {
            nodes: Default::default(),
            minimum_level: 0,
        }
    }
    pub fn insert(&mut self, domain: &str) {
        let is_wildcard = domain.starts_with(".");
        let segments = domain_rseg(domain);
        let n_segments = segments.len();

        let mut head = &mut self.nodes;
        for (i, segment) in segments.iter().copied().enumerate() {
            let node = head
                .entry(segment.to_owned())
                .or_insert_with(|| Node::new(i == n_segments - 2 && is_wildcard, segment));

            if i == n_segments - 2 && is_wildcard {
                return;
            }

            head = &mut node.nodes;
        }
    }

    pub fn lookup(&self, domain: &str) -> Option<String> {
        match self.traverse(domain) {
            None => None,
            Some((fqdn, node)) => {
                return Some(format!("{}{}", if node.wildcard { "." } else { "" }, fqdn))
            }
        }
    }

    pub fn traverse(&self, domain: &str) -> Option<(String, &Node)> {
        let segments = domain_rseg(domain);
        let mut wildcard_match = None;
        let mut head: &NodeList = &self.nodes;

        let mut fqdn = String::new();

        for (i, segment) in segments.iter().copied().enumerate() {
            if let Some(child) = head.get(segment) {
                fqdn = format!(
                    "{}{}{}",
                    segment.to_owned(),
                    if i == 0 { "" } else { "." },
                    fqdn
                );
                head = &child.nodes;
                if i == segments.len() - 1 {
                    return Some((fqdn, child));
                } else if child.wildcard {
                    wildcard_match = Some(child);
                }
            } else {
                break;
            }
        }

        if let Some(m) = wildcard_match {
            Some((fqdn, m))
        } else {
            None
        }
    }
}

fn domain_rseg(domain: &str) -> Vec<&str> {
    domain.rsplit(".").collect::<Vec<&str>>()
}