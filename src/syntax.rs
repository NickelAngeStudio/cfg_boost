// Syntax tree used to generate configuration from TokenStream.

use std::{cell::RefCell, rc::Rc};
use crate::parse::{parse_cfg_predicate, parse_double_operators, parse_node_type, parse_leaf};


/// Syntax tree node used to parse attribute tokens.
/// 
/// # Note
/// Coded without references so code might be weird at some place, but it works.
pub(crate) struct SyntaxTreeNode {
    // Reference to left leaf.
    pub left : Option<Rc<RefCell<SyntaxTreeNode>>>,

    // Reference to right leaf.
    pub right : Option<Rc<RefCell<SyntaxTreeNode>>>,

    /// Type of node
    pub node_type : SyntaxNodeType,

    /// Is the node a !
    pub is_not : bool
}

impl SyntaxTreeNode {

    /// Create a new SyntaxTreeNode from members.
    #[inline(always)]
    pub fn new(left : Option<Rc<RefCell<SyntaxTreeNode>>>, right : Option<Rc<RefCell<SyntaxTreeNode>>>, 
        node_type : SyntaxNodeType, is_not : bool) -> Rc<RefCell<SyntaxTreeNode>> {
        
        Rc::new(RefCell::new(SyntaxTreeNode { left, right, node_type, is_not }))

    }

    /// Generate a syntax tree from tokens.
    /// 
    /// Panic(s)
    /// Will panic is tokens contains any double operators (&&, ||, !!)
    #[inline(always)]
    pub fn generate_tree(tokens: &str) -> Rc<RefCell<SyntaxTreeNode>> {

        match parse_double_operators(tokens){
            Ok(_) => {
                // Root node
                let root = Self::generate_syntax_node(&tokens.trim(), false);

                // Return tree
                root
            },
            Err(err) => panic!("{}", err.message(tokens)),
        }
    }

    /// Parse target attribute from tree into a string.
    /// 
    /// # Panic(s)
    /// Will panic if any SyntaxParseError::InvalidConfigurationPredicate.
    #[inline(always)]
    pub fn target_cfg_to_string(&self) -> String {

        match &self.node_type {
            SyntaxNodeType::ANY(_) => 
                if self.is_not {
                    format!("not(any({}, {}))", self.left.as_ref().unwrap().clone().as_ref().borrow().target_cfg_to_string(), self.right.as_ref().unwrap().clone().as_ref().borrow().target_cfg_to_string())
                } else {
                    format!("any({}, {})", self.left.as_ref().unwrap().clone().as_ref().borrow().target_cfg_to_string(), self.right.as_ref().unwrap().clone().as_ref().borrow().target_cfg_to_string())
                },
            SyntaxNodeType::ALL(_) => 
                if self.is_not {
                    format!("not(all({}, {}))", self.left.as_ref().unwrap().clone().as_ref().borrow().target_cfg_to_string(), self.right.as_ref().unwrap().clone().as_ref().borrow().target_cfg_to_string())
                } else {
                    format!("all({}, {})", self.left.as_ref().unwrap().clone().as_ref().borrow().target_cfg_to_string(), self.right.as_ref().unwrap().clone().as_ref().borrow().target_cfg_to_string())
                },
            SyntaxNodeType::LEAF(label) => 
                match parse_cfg_predicate(label) {
                    Ok(value) => if self.is_not {
                        format!("not({})", value)
                        } else {
                            format!("{}", value)
                        },
                    Err(err) =>  panic!("{}", err.message(label)),
                },
        }
    }

    /// Generate syntax node from tokens.
    /// 
    /// Panic(s)
    /// Will panic! if an operator is missing.
    /// Will panic! if an alias is not found.
    /// Will panic! if tokens is empty.
    #[inline(always)]
    pub fn generate_syntax_node(tokens: &str, is_not : bool) -> Rc<RefCell<SyntaxTreeNode>> {

        // Separate node A from Node B and extract leaf.
        match parse_node_type(tokens){
            Ok(node_type) => 
                {
                    match node_type { // Match node types
                        //If separator, split left and right
                        SyntaxNodeType::ANY(position) => {
                            Self::new(Some(Self::generate_syntax_node(&tokens[0..position], false)), 
                                Some(Self::generate_syntax_node(&tokens[position + 1..], false)),
                                node_type, is_not)
                        },
                        SyntaxNodeType::ALL(position) => {
                            Self::new(Some(Self::generate_syntax_node(&tokens[0..position], false)), 
                                Some(Self::generate_syntax_node(&tokens[position + 1..], false)),
                                node_type, is_not)
                        },
                        SyntaxNodeType::LEAF(_) => parse_leaf(tokens),
                    }
                },
            
            Err(err) => panic!("{}", err.message(tokens)),
        }
    }

    /// Print the syntax tree content with levels.
    #[cfg(debug_assertions)]
    #[inline(always)]
    pub fn print_syntax_tree(tree : Rc<RefCell<SyntaxTreeNode>>) {
        Self::print_syntax_node_level(tree.clone(), "T", 0);
    }

    /// Print syntax node according with level.
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn print_syntax_node_level(node : Rc<RefCell<SyntaxTreeNode>>, tag : &str, level : usize) {
        let node = node.as_ref().borrow();
        println!("\x1b[92m{}{} : {}{:?}\x1b[0m", tag, level, if node.is_not { '!' } else {' '} ,node.node_type);
        match &node.left {
            Some(subnode) => Self::print_syntax_node_level(subnode.clone(), "L", level + 1),
            None => {},
        }
        match &node.right {
            Some(subnode) => Self::print_syntax_node_level(subnode.clone(), "R",  level + 1),
            None => {},
        }
    }
}


/// Type of syntax tree node.
#[derive(Debug)]
pub enum SyntaxNodeType {
    /// A or(|) operation
    ANY(usize),

    /// A and(&) operation
    ALL(usize),

    /// End leaf of the tree
    LEAF(String)
}
