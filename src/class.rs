use ::Modifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemberType {
    Variable,
    Method,
    Constructor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassMember<'a> {
    pub modifiers: Vec<Modifier>,
    pub name: &'a str,
    pub member_type: MemberType,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class<'a> {
    pub name: &'a str,
    pub type_params: Vec<&'a str>,
    pub implements: Vec<&'a str>,
    pub extends: &'a str,
    pub members: Vec<ClassMember<'a>>,
    pub inner_classes: Vec<Class<'a>>,
    pub modifiers: Vec<Modifier>
}

impl<'a> Class<'a> {
    pub fn new_empty() -> Class<'static> {
        Class {
            name: "",
            type_params: Vec::new(),
            implements: Vec::new(),
            extends: "",
            members: Vec::new(),
            inner_classes: Vec::new(),
            modifiers: Vec::new(),
        }
    }
}

