extern crate domain_lookup;
use domain_lookup::DomainLookup;

let mut tree = DomainLookupTree::new();

// Insert some domains
tree.insert(".twitter.com");

// Perform lookups
tree.lookup("api.twitter.com");