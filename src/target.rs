use proc_macro::TokenStream;

use crate::{errors::TargetCfgError, tools::{split_items, extract_modifier}, syntax::{Node, SyntaxTreeNode}};

/// Negative modifier symbol
const NEGATIVE_MODIFIER : char = '!';

/// Panic print modifier symbol
const PRINT_MODIFIER : char = '#';

/// Documentation modifier symbol
const DOC_MODIFIER : char = '?';

/// Debug modifier symbol
const DEBUG_MODIFIER : char = '@';

/// Value override symbol
const OVERRIDE_MODIFIER : char = '*';

/// Exclusive symbol
const EXCLUSIVE_MODIFIER : char = '$';

/// Guard symbol
const GUARD_MODIFIER : char = '%';

/// Always added branch symbol
const ALWAYS_BRANCH : char = '+';

/// Always added branch symbol as string
const ALWAYS_BRANCH_STR : &str = "+";

/// Never added branch symbol
const NEVER_BRANCH : char = '-';

/// Never added branch symbol as string
const NEVER_BRANCH_STR : &str = "-";

/// Exclusive branch symbol
const EXCLUSIVE_BRANCH_STR : &str = "_";


/// cfg_target attributes options.
/// 
/// They are detected by the tail symbols ?, @ and *. They are deactivated using !.
/// 
/// Panic(s)
/// Will panic is any other symbol that !, ?, @, *, # are used.
pub(crate) struct TargetAttributeOption {
    /// Flag that tell if documentation is allowed. cfg!(doc) by default, can be always true with ? or always false with !?.
    pub allow_doc : bool,

    /// Flag that tell if panic print result symbol `#` has been used. Can be cancelled by !#.
    pub is_panic_result : bool,

    /// Flag that tell if branch is debug only or not for debug at all.
    pub debug_only : Option<bool>,

    /// Flag that say that condition is always true. Uses symbol `*`.
    /// None means that it isn't set to force a particular value.
    pub always_this : Option<bool>,
}

impl ToString for TargetAttributeOption {
    fn to_string(&self) -> String {
        format!("allow_doc : `{:?}`, is_panic_result : `{:?}`, debug_only : `{:?}`, always_this : `{:?}`", 
            self.allow_doc, self.is_panic_result, self.debug_only, self.always_this )
    }
}

impl TargetAttributeOption {
    /// Create a new attribute options from symbol tokenstream.
    /// 
    /// Panic(s)
    /// Will panic is illegal characters are used.
    pub fn new(symbol: TokenStream) -> TargetAttributeOption{

        let mut allow_doc = cfg!(doc);
        let mut is_panic_result : bool = false;
        let mut debug_only : Option<bool> = None;
        let mut always_this : Option<bool> = None;

        // Not flag. Activated by !, consumed by other symbol.
        let mut is_not = false;

        for t in symbol.clone() {
            match t {
                proc_macro::TokenTree::Punct(punc) => {
                    match punc.as_char() {
                        NEGATIVE_MODIFIER => is_not = true,   // Activate is_not flag
                        PRINT_MODIFIER => {    // Panic result
                            is_panic_result = !is_not && true;
                            is_not = false; // Consume is_not flag
                        },
                        DOC_MODIFIER => {    // Documentation
                            allow_doc = !is_not && true;
                            is_not = false; // Consume is_not flag
                        },
                        DEBUG_MODIFIER => {    // Debug only
                            debug_only = Some(!is_not && true);
                            is_not = false; // Consume is_not flag
                        },
                        OVERRIDE_MODIFIER => {    // Always true
                            always_this = Some(!is_not && true);
                            is_not = false; // Consume is_not flag
                        },
                        ',' | NEVER_BRANCH | ALWAYS_BRANCH =>  {}, // Ignored
                        _ => // Illegal character
                        panic!("{}", TargetCfgError::InvalidCharacter(punc.as_char()).message(&symbol.to_string())),
                    }
                },
                _ => {},
            }
        }

        TargetAttributeOption{ allow_doc, is_panic_result, debug_only, always_this }

    }

    /// Create a new attribute options from symbol tokenstream and parent target_cfg match.
    /// 
    /// Panic(s)
    /// Will panic is illegal characters are used.
    pub fn from_match(match_opt : &TargetMatchOption, symbol: TokenStream) -> TargetAttributeOption {

        let mut allow_doc = match_opt.allow_doc;
        let mut is_panic_result : bool = match_opt.is_panic_result;   // Copy default value from match
        let mut debug_only : Option<bool> = match_opt.debug_only;
        let mut always_this : Option<bool> = None;

        // Not flag. Activated by !, consumed by other symbol.
        let mut is_not = false;

        for t in symbol.clone() {
            match t {
                proc_macro::TokenTree::Punct(punc) => {
                    match punc.as_char() {
                        NEGATIVE_MODIFIER => is_not = true,   // Activate is_not flag
                        PRINT_MODIFIER => {    // Panic result
                            if ! match_opt.is_panic_result {
                                is_panic_result = !is_not && true; 
                            }
                            is_not = false; // Consume is_not flag                           
                        },
                        DEBUG_MODIFIER => {    // Debug only
                            match match_opt.debug_only {
                                Some(_) => {},
                                None => debug_only = Some(!is_not && true),
                            }
                            is_not = false; // Consume is_not flag
                        },
                        DOC_MODIFIER => {    // Documentation
                            allow_doc = !is_not && true;
                            is_not = false; // Consume is_not flag
                        },
                        OVERRIDE_MODIFIER => {    // Always true
                            always_this = Some(!is_not && true);
                            is_not = false; // Consume is_not flag
                        },
                        ',' | NEVER_BRANCH | ALWAYS_BRANCH =>  {}, // Ignored
                        _ => // Illegal character
                        panic!("{}", TargetCfgError::InvalidCharacter(punc.as_char()).message(&symbol.to_string())),
                    }
                },
                _ => {},
            }
        }

        TargetAttributeOption{ allow_doc, is_panic_result, debug_only, always_this }

    }
}

/// target_cfg matching options.
/// 
/// They are detected by the tail symbols ?, @ and $. They are deactivated using !.
/// 
/// Panic(s)
/// Will panic is any other symbol that !, ?, @, $, # are used.
pub(crate) struct TargetMatchOption {
    /// Flag that tell if documentation is allowed. cfg!(doc) by default, can be always true with ? or always false with !?.
    /// This flag can be overriden locally by TargetAttributeOption if ? is defined.
    pub allow_doc : bool,

    /// Flag that tell if panic result symbol `#` has been used. Can be cancelled by !#.
    /// If true, CANNOT be overriden locally by TargetAttributeOption !#.
    pub is_panic_result : bool,

    /// Flag with symbol `@` that tell if target_cfg is debug only `@` or not `!@`.
    /// If set, CANNOT be overriden locally by TargetAttributeOption.
    pub debug_only : Option<bool>,

    /// Flag that say that target_cfg branchs are exclusives. $ is used to tell it is.
    /// If 2 or more branchs are true, it will panic!. If it is intended, use !$ to be inclusive.
    pub is_exclusive : bool,

    /// Flag that determine if macro is called from inside a function or not.
    pub is_inner_macro : bool,

    /// Flag that determine is branch guard is activated or not. Branch guard panic! when not debug
    /// if `*` or `!*` used or never branch `-` are used. Can be deactivated with `!%`.
    pub activate_branch_guard : bool,
}

impl ToString for TargetMatchOption {
    fn to_string(&self) -> String {
        format!("allow_doc : `{:?}`, is_panic_result : `{:?}`, debug_only : `{:?}`, is_exclusive : `{:?}`, is_inner_macro : `{:?}`, activate_branch_guard : `{:?}`", 
            self.allow_doc, self.is_panic_result, self.debug_only, self.is_exclusive, self.is_inner_macro, self.activate_branch_guard )
    }
}

impl TargetMatchOption {
    /// Create a new match options from symbol tokenstream and target group.
    /// 
    /// Panic(s)
    /// Will panic is illegal characters are used.
    pub fn new(symbol: TokenStream, tg :  &Vec<TargetGroup>) -> TargetMatchOption{

        let mut allow_doc = cfg!(doc);
        let mut is_panic_result = false;
        let mut is_exclusive  = true;
        let mut debug_only : Option<bool> = None;
        let mut activate_branch_guard = true;

        // Not flag. Activated by !, consumed by other symbol.
        let mut is_not = false;

        for t in symbol.clone() {
            match t {
                proc_macro::TokenTree::Punct(punc) => {
                    match punc.as_char() {
                        NEGATIVE_MODIFIER => is_not = true,   // Activate is_not flag
                        PRINT_MODIFIER => {    // Panic result
                            is_panic_result = !is_not && true;
                            is_not = false; // Consume is_not flag
                        },
                        DEBUG_MODIFIER => {    // Debug only
                            debug_only = Some(!is_not && true);
                            is_not = false; // Consume is_not flag
                        },
                        DOC_MODIFIER => {    // Documentation
                            allow_doc = !is_not && true;
                            is_not = false; // Consume is_not flag
                        },
                        EXCLUSIVE_MODIFIER => {    // Exclusive
                            is_exclusive = !is_not && true;
                            is_not = false; // Consume is_not flag
                        },
                        GUARD_MODIFIER => {     // Guard modifier
                            activate_branch_guard = !is_not && true;
                            is_not = false; // Consume is_not flag
                        }
                        ',' =>  {}, // Ignore separator
                        _ => // Illegal character
                        panic!("{}", TargetCfgError::InvalidCharacter(punc.as_char()).message(&symbol.to_string())),
                    }
                },
                _ => {},
            }
        }

        TargetMatchOption{ allow_doc, is_panic_result, debug_only, is_exclusive, is_inner_macro : Self::is_inner_macro(tg), activate_branch_guard }

    }

    /// Returns true if macro was called inside a function.
    /// 
    /// This function tries to detect `let` and flow of control keywords to determine if inner or not.
    fn is_inner_macro(tg : &Vec<TargetGroup>) -> bool {

        for g in tg  {
            for t in g.item.clone() {
                match t {
                    proc_macro::TokenTree::Ident(ident) => match ident.to_string().as_str() {
                        "let" | "if" | "else" | "loop" | "break" | "while" | "for" | "match" | "println"   => return true,    // Those keyword are only found inside functions.
                        _ => {},
                    },
                    _ => {},
                }
                
            }
        }

        false
    }
}



/// Keep track of active target_cfg branchs.
pub(crate) struct TargetActiveCounter {
    is_exclusive : bool,

    counter : usize,
}

impl TargetActiveCounter {
    /// Create a new TargetActiveCounter referencing is_exclusive option.
    pub fn new(options : &TargetMatchOption) -> TargetActiveCounter {
        TargetActiveCounter { is_exclusive : options.is_exclusive, counter: 0 }
    }

    /// Increment active counter.
    /// 
    /// Panic(s)
    /// Will panic if counter > 1 and is_exclusive is true.
    pub fn inc(&mut self) {
        self.counter = self.counter + 1;

        if self.is_exclusive && self.counter > 1 {  // Panic if exclusive and more than 1 branch active.
            panic!("{}", TargetCfgError::TargetCfgIsExclusive.message(""));
            
        }
    }

    /// Get counter.
    pub fn get_counter(&self) -> usize {
        self.counter
    }
}

/// Struct used to contain an item with attributes.
pub(crate) struct TargetGroup {
    pub attr : TokenStream,
    pub item : TokenStream,
}

impl ToString for TargetGroup {
    /// Transform self into string.
    fn to_string(&self) -> String {
        format!("attr : {}, item : {}", self.attr.to_string(), self.item.to_string())
    }
}

impl TargetGroup {
    /// Create a new TargetGroup from attributes and item.
    pub fn new(attr : TokenStream, item : TokenStream) -> TargetGroup {
        TargetGroup { attr, item }
    }
    /// Extract target groups into a vector.
    pub fn extract(source : TokenStream) -> Vec<TargetGroup> {

        // Flag used to tell if exclusive branch was added. Must always be last branch.
        let mut exclusive_branch_added = false;

        // Tell is next block is attributes
        let mut is_block_attr : bool = true;
        let mut tg : Vec<TargetGroup> = Vec::new();

        let mut attr : TokenStream = TokenStream::new();

        for token in source {

            match token {
                // Normal branch
                proc_macro::TokenTree::Group(grp) => {
                    

                    if is_block_attr {
                        if exclusive_branch_added { // Panic since exclusive branch isn't last.
                            panic!("{}", TargetCfgError::ExclusiveBranchNotLast.message(""));
                        }
                        attr = grp.stream();

                        // Exclusive branch guard when used between ()
                        match grp.stream().to_string().find(EXCLUSIVE_BRANCH_STR) {
                            Some(_) => exclusive_branch_added = true,
                            None => {},
                        }
                    } else {
                        tg.push(TargetGroup::new(attr, grp.stream()));
                        attr = TokenStream::new();
                    }
                    is_block_attr = !is_block_attr;

                },

                // Special exclusive branch `_`.
                proc_macro::TokenTree::Ident(ident) => {
                    if ident.to_string().eq(EXCLUSIVE_BRANCH_STR) {
                        if is_block_attr {
                            exclusive_branch_added = true;
                            attr = EXCLUSIVE_BRANCH_STR.parse().unwrap();
                            is_block_attr = !is_block_attr;
                        }
                    }
                },

                // Special always included branch `+` or `-`.
                proc_macro::TokenTree::Punct(punct) => 
                    match punct.as_char() {
                        ALWAYS_BRANCH => if is_block_attr {
                            attr = String::from(ALWAYS_BRANCH).parse().unwrap();
                            is_block_attr = !is_block_attr;
                        },
                        NEVER_BRANCH => if is_block_attr {
                            attr = String::from(NEVER_BRANCH).parse().unwrap();
                            is_block_attr = !is_block_attr;
                        },
                        _ => {}
                },
                _ => {},
                
            }
        }

        tg
    }
}


/// Generate target cfg content.
/// 
/// Panic(s)
/// Will panic is more than 1 branch is activated and exclusive.
#[inline(always)]
pub(crate) fn generate_target_cfg_content(tg : &Vec<TargetGroup>, options : &TargetMatchOption, panic_str : &mut String) -> TokenStream {

    // TokenStream that accumulate content
    let mut content = TokenStream::new();

    // 1. Create active branchs counter
    let mut branch_cpt = TargetActiveCounter::new(&options);

    // 2. For each group
    for g in tg {

        // 2.1. Extract modifiers from attributes 
        let (modifier_attr, attr) = extract_modifier(g.attr.clone());

        // 2.2. Panic! if branch attributes is empty.
        if attr.is_empty() {
            panic!("{}", TargetCfgError::EmptyBranch.message(&g.item.to_string()));
        }

        // 2.3. Generate attributes options from modifiers and match options
        let opt_attr = TargetAttributeOption::from_match(&options, modifier_attr.clone());

        // 2.4. Generate syntax tree from attributes according to branch type.
        let syntax_tree = match attr.to_string().as_str() {
            // Special always added branch
            ALWAYS_BRANCH_STR => SyntaxTreeNode::empty(true),

            // Special never added branch
            NEVER_BRANCH_STR => {
                // Branch guard prevent never branch `-` when not in debug.
                if !cfg!(debug_assertions) && options.activate_branch_guard  {
                    panic!("{}", TargetCfgError::GuardBlockNeverAddedBranchNotDebug.message(""))
                }
                SyntaxTreeNode::empty(false)
            },

            // Special exclusive branch
            EXCLUSIVE_BRANCH_STR => SyntaxTreeNode::empty(branch_cpt.get_counter() == 0),

            // Normal branch
            _ => SyntaxTreeNode::generate(attr.clone()),
        };
        
        if options.is_panic_result {   // Push attr panic result 
            panic_str.push_str(&format!("\n{}", target_cfg_attr_panic_message(g.attr.clone(), &opt_attr, syntax_tree.clone())));
        }
        
        // 2.5. Evaluate if value is overriden.
        match opt_attr.always_this {
            Some(is_activated) => {
                 // Branch guard prevent always true `*` or false `!*` when not in debug.
                 if !cfg!(debug_assertions) && options.activate_branch_guard  {
                    panic!("{}", TargetCfgError::GuardBlockAlwaysValueBranchNotDebug.message(""));
                }

                if is_activated {
                    content.extend(target_cfg_activate_branch(&mut branch_cpt, &options, syntax_tree.clone(), g.item.clone()));
                }
            },
            // 2.6 Evaluate syntax_tree
            None => if options.allow_doc || syntax_tree.evaluate() {
                content.extend(target_cfg_activate_branch(&mut branch_cpt, &options, syntax_tree.clone(), g.item.clone()));

            },
        }
    }

    // Branch guard prevent target_cfg! that has no branchs activated.
    if options.activate_branch_guard && branch_cpt.get_counter() == 0  {
        panic!("{}", TargetCfgError::GuardNoBranchActivated.message(""));
    }

    content
}

/// Activate a target_cfg! branch from parameters.
#[inline(always)]
pub(crate) fn target_cfg_activate_branch(branch_cpt : &mut TargetActiveCounter, options : &TargetMatchOption, syntax_tree : Node, item: TokenStream) -> TokenStream {
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
pub(crate) fn generate_cfg_target_content(options : &TargetAttributeOption, syntax_tree : Node, item: TokenStream) -> TokenStream {

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
pub(crate) fn cfg_target_activate(options : &TargetAttributeOption, syntax_tree : Node, item : TokenStream) -> TokenStream{
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
pub(crate) fn cfg_target_attr_panic_message(attr: TokenStream, options : &TargetAttributeOption, syntax_tree : Node, content : TokenStream) -> String {
    format!("\nATTR          {}\nOPTIONS       {}\nTO_STRING     {}\nLEAF_EVAL     {}\nTREE_EVAL     {}\nEVAL_OVER     {:?}\nCONTENT\n{}",
    attr.to_string(), options.to_string(), syntax_tree.to_string(), syntax_tree.leaf_node_eval_to_string(), syntax_tree.evaluate(), options.always_this,
    content.to_string())
}

/// This function create the panic message for target_cfg!.
#[inline(always)]
pub(crate) fn target_cfg_attr_panic_message(attr: TokenStream, options : &TargetAttributeOption, syntax_tree : Node) -> String {
    format!("\nBRANCH        {}\nOPTIONS       {}\nTO_STRING     {}\nLEAF_EVAL     {}\nTREE_EVAL     {}\nEVAL_OVER     {:?}",
    attr.to_string(), options.to_string(), syntax_tree.to_string(), syntax_tree.leaf_node_eval_to_string(), syntax_tree.evaluate(), options.always_this)
}