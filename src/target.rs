use proc_macro::TokenStream;

use crate::{errors::TargetCfgError, tools::{split_items, extract_modifier}, syntax::{Node, SyntaxTreeNode}, modifiers::{TargetBranch, TargetCfgModifier, TargetActiveCounter, TargetAttributeModifier, TargetBranchType}};

/// Generate target cfg content.
/// 
/// Panic(s)
/// Will panic is more than 1 branch is activated and exclusive.
#[inline(always)]
pub(crate) fn generate_target_cfg_content(branchs : &Vec<TargetBranch>, options : &TargetCfgModifier, panic_str : &mut String) -> TokenStream {

    // TokenStream that accumulate content
    let mut content = TokenStream::new();

    // 1. Create active branchs counter
    let mut branch_cpt = TargetActiveCounter::new(&options);

    // 2. For each group
    for branch in branchs {

        // 2.1. Extract modifiers from attributes 
        let (modifier_attr, attr) = extract_modifier(branch.attr.clone());

        // 2.2. Panic! if branch attributes is empty.
        if attr.is_empty() {    
            panic!("{}", TargetCfgError::EmptyBranch.message(&branch.content.to_string()));
        }

        // 2.3. Generate attributes options from modifiers and match options
        let opt_attr = TargetAttributeModifier::from_match(&options, modifier_attr.clone());

        // 2.4. Generate syntax tree from attributes according to branch type.
        branch_cpt.set_branch_type(branch.branch_type);
        let syntax_tree = match branch.branch_type {
            TargetBranchType::Normal =>  SyntaxTreeNode::generate(attr.clone()),    // Normal evaluated branch
            TargetBranchType::Added => SyntaxTreeNode::empty(branch_cpt.get_counter() > 0), // Added only if a branch was activated before.
            TargetBranchType::Exclusive => SyntaxTreeNode::empty(branch_cpt.get_counter() == 0), // Added only if no branch activated before.
        };
        
        // 2.5. Push debug string if #.
        if options.is_panic_result {   // Push attr panic result 
            panic_str.push_str(&format!("\n{}", target_cfg_attr_panic_message(branch.attr.clone(), &opt_attr, syntax_tree.clone())));
        }
        
        // 2.6. Evaluate if value is overriden.
        match opt_attr.always_this {
            Some(is_activated) => {
                 // Branch guard prevent always true `*` or false `!*` when not in debug.
                 if !cfg!(debug_assertions) && options.activate_branch_guard  {
                    panic!("{}", TargetCfgError::GuardBlockAlwaysValueBranchNotDebug.message(""));
                }

                if is_activated {
                    content.extend(target_cfg_activate_branch(&mut branch_cpt, &options, syntax_tree.clone(), branch.content.clone()));
                }
            },
            // 2.7 Evaluate syntax_tree
            None => if options.allow_doc || syntax_tree.evaluate() {
                content.extend(target_cfg_activate_branch(&mut branch_cpt, &options, syntax_tree.clone(), branch.content.clone()));

            },
        }
    }

    // 2.8. Validate branch counter to make sure to respect control.
    branch_cpt.validate();

    content
}

/// Activate a target_cfg! branch from parameters.
#[inline(always)]
pub(crate) fn target_cfg_activate_branch(branch_cpt : &mut TargetActiveCounter, options : &TargetCfgModifier, syntax_tree : Node, item: TokenStream) -> TokenStream {
    // TokenStream that accumulate content
    let mut content = TokenStream::new();

    // 1. Increment active branch.
    branch_cpt.inc();

    // 2. Verify if macro is inside
    if options.is_inner_macro {
        // 2.1. Add item as is in content
        content.extend(item);
    } else {
        // 2.2. Create attr header
        let attr_ts = format!("#[cfg_attr(docsrs, doc(cfg({})))]", syntax_tree.to_string()).parse::<TokenStream>().unwrap();

        // 2.3. Split item into vector of items
        let items = split_items(item.clone());

        // 2.4.. For each item in vector of items
        for item in items {
            // 2.4.1. Add attr header.
            content.extend(attr_ts.clone()); 

            // 2.4.2. Add item to content
            content.extend(item);
        }
    }

    // 3. Return content generated
    content

}

/// Generate cfg_target tokenstream content.
#[inline(always)]
pub(crate) fn generate_cfg_target_content(options : &TargetAttributeModifier, syntax_tree : Node, item: TokenStream) -> TokenStream {

    // Evaluate if value is overriden.
    match options.always_this {
        // Value is overriden, use provided value
        Some(is_activated) => if is_activated {
            cfg_target_activate(&options, syntax_tree.clone(), item)
        } else {
            TokenStream::default()
        },
        // Value not overriden, evaluate doc and tree
        None => if options.allow_doc || syntax_tree.evaluate() {
            cfg_target_activate(&options, syntax_tree.clone(), item)
        } else {
            TokenStream::default()
        },
    }

}

/// Activate tokenstream.
#[inline(always)]
pub(crate) fn cfg_target_activate(options : &TargetAttributeModifier, syntax_tree : Node, item : TokenStream) -> TokenStream{
    let mut content = TokenStream::new();

    if options.allow_doc {
        // 1. Extend cfg_attr header for documentation
        content.extend(format!("#[cfg_attr(docsrs, doc(cfg({})))]", syntax_tree.to_string()).parse::<TokenStream>().unwrap());
    }

    // 2. Add item to content
    content.extend(item);

    // 3. Write content to stream
    content        
}

/// This function create the panic message for attributes.
#[inline(always)]
pub(crate) fn cfg_target_attr_panic_message(attr: TokenStream, options : &TargetAttributeModifier, syntax_tree : Node, content : TokenStream) -> String {
    format!("\nATTR          {}\nOPTIONS       {}\nTO_STRING     {}\nLEAF_EVAL     {}\nTREE_EVAL     {}\nEVAL_OVER     {:?}\nCONTENT\n{}",
    attr.to_string(), options.to_string(), syntax_tree.to_string(), syntax_tree.leaf_node_eval_to_string(), syntax_tree.evaluate(), options.always_this,
    content.to_string())
}

/// This function create the panic message for target_cfg!.
#[inline(always)]
pub(crate) fn target_cfg_attr_panic_message(attr: TokenStream, options : &TargetAttributeModifier, syntax_tree : Node) -> String {
    format!("\nBRANCH        {}\nOPTIONS       {}\nTO_STRING     {}\nLEAF_EVAL     {}\nTREE_EVAL     {}\nEVAL_OVER     {:?}",
    attr.to_string(), options.to_string(), syntax_tree.to_string(), syntax_tree.leaf_node_eval_to_string(), syntax_tree.evaluate(), options.always_this)
}