use proc_macro::TokenStream;

use crate::errors::TargetCfgError;


/// Negative modifier symbol
pub(crate) const NEGATIVE_MODIFIER : char = '!';

/// Panic print modifier symbol
pub(crate) const PRINT_MODIFIER : char = '#';

/// Documentation modifier symbol
pub(crate) const DOC_MODIFIER : char = '?';

/// Debug modifier symbol
pub(crate) const DEBUG_MODIFIER : char = '@';

/// Value override symbol
pub(crate) const OVERRIDE_MODIFIER : char = '*';

/// Guard symbol
pub(crate) const GUARD_MODIFIER : char = '$';

/// Added branch symbol. (Added only if 1 branch before was added)
pub(crate) const ADDED_BRANCH : char = '+';

/// Exclusive branch symbol
pub(crate) const EXCLUSIVE_BRANCH : char = '_';

/// Operator equal
pub(crate) const EQUAL_OP : char = '=';

/// Operator greater
pub(crate) const GREATER_OP : char = '>';

/// Operator less than
pub(crate) const LESS_OP : char = '<';


/// cfg_target attributes modifiers.
/// 
/// They are detected by the tail symbols ?, @ and *. They are deactivated using !.
/// 
/// Panic(s)
/// Will panic is any other symbol that !, ?, @, *, # are used.
pub(crate) struct TargetAttributeModifier {
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

impl ToString for TargetAttributeModifier {
    fn to_string(&self) -> String {
        format!("allow_doc : `{:?}`, is_panic_result : `{:?}`, debug_only : `{:?}`, always_this : `{:?}`", 
            self.allow_doc, self.is_panic_result, self.debug_only, self.always_this )
    }
}

impl TargetAttributeModifier {
    /// Create a new attribute modifier from symbol tokenstream.
    /// 
    /// Panic(s)
    /// Will panic is illegal characters are used.
    pub fn new(symbol: TokenStream) -> TargetAttributeModifier{

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
                        ',' =>  {}, // Ignored
                        _ => // Illegal character
                        panic!("{}", TargetCfgError::InvalidCharacter(String::from(punc.as_char())).message(&symbol.to_string())),
                    }
                },
                _ => {},
            }
        }

        TargetAttributeModifier{ allow_doc, is_panic_result, debug_only, always_this }

    }

    /// Create a new attribute modifier from symbol tokenstream and parent target_cfg match.
    /// 
    /// Panic(s)
    /// Will panic is illegal characters are used.
    pub fn from_match(match_opt : &TargetCfgModifier, symbol: TokenStream) -> TargetAttributeModifier {

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
                        ',' =>  {}, // Ignored
                        _ => // Illegal character
                        panic!("{}", TargetCfgError::InvalidCharacter(String::from(punc.as_char())).message(&symbol.to_string())),
                    }
                },
                _ => {},
            }
        }

        TargetAttributeModifier{ allow_doc, is_panic_result, debug_only, always_this }

    }
}

/// target_cfg macro modifiers.
/// 
/// They are detected by the tail symbols ?, @ and $. They are deactivated using !.
/// 
/// Panic(s)
/// Will panic is any other symbol that !, ?, @, $, # are used.
pub(crate) struct TargetCfgModifier {
    /// Flag that tell if documentation is allowed. cfg!(doc) by default, can be always true with ? or always false with !?.
    /// This flag can be overriden locally by TargetAttributeOption if ? is defined.
    pub allow_doc : bool,

    /// Flag that tell if panic result symbol `#` has been used. Can be cancelled by !#.
    /// If true, CANNOT be overriden locally by TargetAttributeOption !#.
    pub is_panic_result : bool,

    /// Flag with symbol `@` that tell if target_cfg is debug only `@` or not `!@`.
    /// If set, CANNOT be overriden locally by TargetAttributeOption.
    pub debug_only : Option<bool>,

    /// Flag that determine if macro is called from inside a function or not.
    pub is_inner_macro : bool,

    /// Flag that determine is branch guard is activated or not. Branch guard panic! when not debug
    /// if `*` or `!*` used or never branch `-` are used. Can be deactivated with `!%`.
    pub activate_branch_guard : bool,

    /// Comparison operator. Deducted via comparison symbol before an integer.
    pub operator : TargetActiveComparisonOperator,

    /// Control value to be compared with when counting actives branches.  Branches must equal 1 by default.
    pub control : usize,
}

impl ToString for TargetCfgModifier {
    fn to_string(&self) -> String {
        format!("allow_doc : `{:?}`, is_panic_result : `{:?}`, debug_only : `{:?}`, is_inner_macro : `{:?}`, activate_branch_guard : `{:?}`, operator : `{}`, control : `{}`", 
            self.allow_doc, self.is_panic_result, self.debug_only, self.is_inner_macro, self.activate_branch_guard, self.operator.to_string(), self.control )
    }
}

impl TargetCfgModifier {
    /// Create a new targetcfg! modifier from symbol tokenstream and target group.
    /// 
    /// Panic(s)
    /// Will panic is illegal characters are used.
    pub fn new(symbol: TokenStream, tg :  &Vec<TargetBranch>) -> TargetCfgModifier{

        let mut allow_doc = cfg!(doc);
        let mut is_panic_result = false;
        let mut debug_only : Option<bool> = None;
        let mut activate_branch_guard = true;
        // Branches must equal 1 by default.
        let mut operator : TargetActiveComparisonOperator = TargetActiveComparisonOperator::Equal;
        let mut control : usize = 1;

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
                        GUARD_MODIFIER => {     // Guard modifier
                            activate_branch_guard = !is_not && true;
                            is_not = false; // Consume is_not flag
                        },
                        EQUAL_OP => {   // Equal operator
                            match operator {
                                // Becomes >= or <= if > or < was set before
                                TargetActiveComparisonOperator::GreaterThan => {
                                    operator = TargetActiveComparisonOperator::GreaterEqualThan;
                                    is_not = false;
                                },
                                TargetActiveComparisonOperator::LessThan => {
                                    operator = TargetActiveComparisonOperator::LessEqualThan;
                                    is_not = false;
                                },
                                _ => {
                                    if is_not {
                                        operator = TargetActiveComparisonOperator::NotEqual;
                                    } else {
                                        operator = TargetActiveComparisonOperator::Equal;
                                    }
                                    is_not = false;
                                },
                            }
                            

                        },
                        GREATER_OP => {
                            operator = TargetActiveComparisonOperator::GreaterThan;
                            is_not = false;
                        },
                        LESS_OP => {
                            operator = TargetActiveComparisonOperator::LessThan;
                            is_not = false;
                        }
                        ',' =>  {}, // Ignore separator
                        _ => // Illegal character
                        panic!("{}", TargetCfgError::InvalidCharacter(String::from(punc.as_char())).message(&symbol.to_string())),
                    }
                },
                // Control value
                proc_macro::TokenTree::Literal(lit) => {
                    match lit.to_string().parse::<usize>(){
                        Ok(value) => control = value,
                        Err(_) => panic!("{}", TargetCfgError::InvalidCharacter(lit.to_string()).message(&symbol.to_string())),
                    }
                },
                _ => is_not = false, // Consume is_not flag,
                
            }
        }

        TargetCfgModifier{ allow_doc, is_panic_result, debug_only, is_inner_macro : Self::is_inner_macro(tg), activate_branch_guard, operator, control }

    }

    /// Returns true if macro was called inside a function.
    /// 
    /// This function tries to detect flow of control keywords to determine if in a function or not.
    fn is_inner_macro(tg : &Vec<TargetBranch>) -> bool {

        for g in tg  {
            for t in g.content.clone() {
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

/// Enumeration of counter modifier operator.
#[derive(Debug, Clone, Copy)]
pub enum TargetActiveComparisonOperator {
    /// Equal =
    Equal, 

    /// Not Equal !=
    NotEqual,

    /// Greater than >
    GreaterThan,

    /// Less than <
    LessThan,
    
    /// Greater than or equal to >=
    GreaterEqualThan,

    /// Less than or equal to <=
    LessEqualThan,
}

impl ToString for TargetActiveComparisonOperator{
    fn to_string(&self) -> String {
        String::from(match self {
            TargetActiveComparisonOperator::Equal => "=",
            TargetActiveComparisonOperator::NotEqual => "!=",
            TargetActiveComparisonOperator::GreaterThan => ">",
            TargetActiveComparisonOperator::LessThan => "<",
            TargetActiveComparisonOperator::GreaterEqualThan => ">=",
            TargetActiveComparisonOperator::LessEqualThan => "<=",
        })
    }
}

impl TargetActiveComparisonOperator {
    /// Compare operator control VS value and return boolean result.
    pub fn compare(&self, value : usize, control : usize) -> bool {
        match self {
            TargetActiveComparisonOperator::Equal => value == control,
            TargetActiveComparisonOperator::NotEqual => value != control,
            TargetActiveComparisonOperator::GreaterThan => value > control,
            TargetActiveComparisonOperator::LessThan => value < control,
            TargetActiveComparisonOperator::GreaterEqualThan => value >= control,
            TargetActiveComparisonOperator::LessEqualThan => value <= control,
        }
    }
}

/// Keep track of active target_cfg branchs.
pub(crate) struct TargetActiveCounter {
    /// Branch type counted.
    branch_type : TargetBranchType,

    /// Normal branch counter
    counter : usize,

    /// Comparison operator
    operator : TargetActiveComparisonOperator,

    /// Control value to compare counter with
    control : usize,
}

impl TargetActiveCounter {
    /// Create a new TargetActiveCounter referencing is_exclusive option.
    pub fn new(options : &TargetCfgModifier) -> TargetActiveCounter {
        TargetActiveCounter { branch_type : TargetBranchType::Normal, counter: 0, operator: options.operator, control: options.control }
    }

    /// Increment active counter.
    /// 
    /// Panic(s)
    /// Will panic if counter > 1 and is_exclusive is true.
    pub fn inc(&mut self) {
        match self.branch_type {
            TargetBranchType::Normal | TargetBranchType::Exclusive => {     // Notmal and exclusives are counted.
                self.counter = self.counter + 1;

                
            },
            TargetBranchType::Added => {},     // Added branch are not counted (they are added only if other branch are activated).
        }
    }

    /// Validate that counter compare to control is true. Else will panic!.
    pub fn validate(&self) {
        if ! self.operator.compare(self.counter, self.control) {  // Compare with operator
            panic!("{}", TargetCfgError::ActiveBranchCountError(self.counter, self.operator, self.control).message(""));
            
        }
    }

    /// Get counter.
    pub fn get_counter(&self) -> usize {
        self.counter
    }

    /// Set branch type counted.
    pub fn set_branch_type(&mut self, branch_type : TargetBranchType) {
        self.branch_type = branch_type;
    }
    
}


/// Enumeration of possible branch types
#[derive(Debug, Clone, Copy)]
pub(crate) enum TargetBranchType {
    /// Normal branch with predicates. Is counted via TargetActiveCounter.
    Normal,

    /// Special branch that is added only if another branch was activated. Is NOT counted via TargetActiveCounter.
    Added,

    /// Branch that only activate if no other branch were activated. Is counted via TargetActiveCounter.
    Exclusive,
}


/// Struct used that contains a branch type, it's attributes and content.
pub(crate) struct TargetBranch {
    pub branch_type : TargetBranchType,
    pub attr : TokenStream,
    pub content : TokenStream,
}

impl ToString for TargetBranch {
    /// Transform self into string.
    fn to_string(&self) -> String {
        format!("branch_type : {:?}, attr : {}, content : {}", self.branch_type, self.attr.to_string(), self.content.to_string())
    }
}

impl TargetBranch {
    /// Create a new TargetGroup from attributes and item.
    pub fn new(branch_type : TargetBranchType, attr : TokenStream, content : TokenStream) -> TargetBranch {
        TargetBranch { branch_type, attr, content }
    }

    /// Extract target branchs into a vector.
    pub fn extract(source : TokenStream) -> Vec<TargetBranch> {

        let mut branchs : Vec<TargetBranch> = Vec::new();

        // Used to store branch attributes
        let mut branch_type = TargetBranchType::Normal;
        let mut attr : TokenStream = TokenStream::new();

        // To validate that _ is last branch
        let mut last_branch = false;

        for token in source {
            match token.clone() {
                proc_macro::TokenTree::Group(grp) => {
                    match grp.delimiter() {
                        proc_macro::Delimiter::Parenthesis => { // Normal branch attributes
                            if last_branch { // Panic since exclusive branch isn't last.
                                panic!("{}", TargetCfgError::ExclusiveBranchNotLast.message(""));
                            }
                            branch_type = TargetBranchType::Normal;  // Special exclusive branch
                            attr.extend(grp.stream());  // Add branch attributes
                        },
                        proc_macro::Delimiter::Brace => {   // Branch content
                            branchs.push(TargetBranch::new(branch_type, attr, grp.stream()));
                            attr = TokenStream::new();  // Reset attributes
                        },
                        proc_macro::Delimiter::Bracket => { // Special branch modifier
                            if last_branch { // Panic since exclusive branch isn't last.
                                panic!("{}", TargetCfgError::ExclusiveBranchNotLast.message(""));
                            }
                            attr.extend(TokenStream::from(token));  // Add modifier to branch attributes
                        },
                        _ => {},    // Ignore rest
                       
                    }
                },
                proc_macro::TokenTree::Ident(ident) => {
                    if ident.to_string().eq(&String::from(EXCLUSIVE_BRANCH)) {
                        branch_type = TargetBranchType::Exclusive;  // Special exclusive branch
                        attr.extend(String::from(EXCLUSIVE_BRANCH).parse::<TokenStream>().unwrap());  // Add branch attributes
                        last_branch = true;     // Activate that this is last branch
                    }
                },
                proc_macro::TokenTree::Punct(punct) => match punct.as_char() {
                        ADDED_BRANCH => {
                            if last_branch { // Panic since exclusive branch isn't last.
                                panic!("{}", TargetCfgError::ExclusiveBranchNotLast.message(""));
                            }
                            branch_type = TargetBranchType::Added;
                            attr.extend(String::from(ADDED_BRANCH).parse::<TokenStream>().unwrap());  // Add branch attributes
                        }, // Special added branch
                        _ => {},    // Ignore rest
                    },
                    
                proc_macro::TokenTree::Literal(_) => {},    // Literal are ignored.
            }

        }

        branchs

        /*
        // Flag used to tell if exclusive branch was added. Must always be last branch.
        let mut exclusive_branch_added = false;

        // Tell is next block is attributes
        let mut is_block_attr : bool = true;
        let mut tg : Vec<TargetBranch> = Vec::new();

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
                        tg.push(TargetBranch::new(attr, grp.stream()));
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
                        ADDED_BRANCH => if is_block_attr {
                            attr = String::from(ADDED_BRANCH).parse().unwrap();
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
        */
    }
}