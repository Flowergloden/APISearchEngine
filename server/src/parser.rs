use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

macro_rules! define {
    ($($name:ident),*) => {
        $(let mut $name = String::new();)*
    };
}

#[derive(thiserror::Error, Debug)]
pub enum TagError {
    #[error("Fail to get pairs due to {0}.")]
    PairsError(#[from] pest::error::Error<Rule>),
}

type Result<T> = std::result::Result<T, TagError>;

#[derive(Parser)]
#[grammar = "xml.pest"]
pub struct TagParser;

impl TagParser {
    pub fn parse_doc(&self, input: &str) -> Result<TagContent> {
        let mut ctx = TagContent::new();
        let pairs = Self::parse(Rule::Doc, input)?;
        for pair in pairs {
            match pair.as_rule() {
                Rule::Doc => self.doc(&mut ctx, pair.into_inner()),
                _ => unreachable!(),
            }
        }
        Ok(ctx)
    }

    pub fn doc(&self, ctx: &mut TagContent, pairs: Pairs<Rule>) {
        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI => {}
                Rule::FileItem => self.file_item(ctx, pair.into_inner()),
                Rule::ClassItem => self.class_item(ctx, pair.into_inner()),
                Rule::NamespaceItem => self.namespace_item(ctx, pair.into_inner()),
                _ => unreachable!(),
            }
        }
    }

    fn file_item(&self, ctx: &mut TagContent, pairs: Pairs<Rule>) {
        define! { name, filename, namespace }
        for pair in pairs {
            match pair.as_rule() {
                Rule::NameField => {
                    name = self.field(pair.into_inner());
                }
                Rule::FilenameField => {
                    filename = self.field(pair.into_inner());
                }
                Rule::NamespaceField => {
                    namespace = self.field(pair.into_inner());
                }
                _ => unreachable!(),
            }
        }
        ctx.files.push(Tag::File {
            name,
            filename,
            namespace,
        });
    }

    fn class_item(&self, ctx: &mut TagContent, pairs: Pairs<Rule>) {
        define! { name, filename }
        let mut of_namespace = None;
        for pair in pairs {
            match pair.as_rule() {
                Rule::NameField => {
                    let field = self.field(pair.into_inner());
                    name = field;
                    of_namespace = ctx
                        .namespaces
                        .iter()
                        .find(|x| {
                            if let Tag::Namespace { classes, .. } = x {
                                classes.contains(&name)
                            } else {
                                false
                            }
                        })
                        .map(|x| x.name());
                }
                Rule::FilenameField => {
                    let field = self.field(pair.into_inner());
                    filename = field;
                }
                Rule::ClassField => { /* nop */ }
                Rule::FunctionField => self.function_field(
                    ctx,
                    pair.into_inner(),
                    of_namespace.clone(),
                    Some(name.clone()),
                ),
                Rule::VariableField => self.variable_field(
                    ctx,
                    pair.into_inner(),
                    of_namespace.clone(),
                    Some(name.clone()),
                ),
                _ => unreachable!(),
            }
        }
        ctx.class.push(Tag::Class {
            name,
            filename,
            of_namespace,
        })
    }

    fn namespace_item(&self, ctx: &mut TagContent, pairs: Pairs<Rule>) {
        define! { name, filename }
        let mut classes = vec![];
        for pair in pairs {
            match pair.as_rule() {
                Rule::NameField => {
                    name = self.field(pair.into_inner());
                }
                Rule::FilenameField => {
                    filename = self.field(pair.into_inner());
                }
                Rule::ClassField => {
                    classes.push(self.field(pair.into_inner()));
                }
                Rule::NamespaceField => { /* nop */ }
                Rule::FunctionField => {
                    self.function_field(ctx, pair.into_inner(), Some(name.clone()), None)
                }
                Rule::VariableField => {
                    self.variable_field(ctx, pair.into_inner(), Some(name.clone()), None)
                }
                _ => unreachable!(),
            }
        }
        ctx.namespaces.push(Tag::Namespace {
            name,
            filename,
            classes,
        })
    }

    fn function_field(
        &self,
        ctx: &mut TagContent,
        pairs: Pairs<Rule>,
        of_namespace: Option<String>,
        of_class: Option<String>,
    ) {
        define! { ty, name, af, a, args }
        for pair in pairs {
            match pair.as_rule() {
                Rule::TypeField => {
                    ty = self.field(pair.into_inner());
                }
                Rule::NameField => {
                    name = self.field(pair.into_inner());
                }
                Rule::AnchorfileField => {
                    af = self.field(pair.into_inner());
                }
                Rule::AnchorField => {
                    a = self.field(pair.into_inner());
                }
                Rule::ArglistField => {
                    args = self.field(pair.into_inner());
                }
                _ => unreachable!(),
            }
        }
        ctx.functions.push(Tag::Function {
            name,
            ty,
            af,
            a,
            args,
            of_class,
            of_namespace,
        })
    }

    fn variable_field(
        &self,
        ctx: &mut TagContent,
        pairs: Pairs<Rule>,
        of_namespace: Option<String>,
        of_class: Option<String>,
    ) {
        define! { ty, name, af, a, args }
        for pair in pairs {
            match pair.as_rule() {
                Rule::TypeField => {
                    ty = self.field(pair.into_inner());
                }
                Rule::NameField => {
                    name = self.field(pair.into_inner());
                }
                Rule::AnchorfileField => {
                    af = self.field(pair.into_inner());
                }
                Rule::AnchorField => {
                    a = self.field(pair.into_inner());
                }
                Rule::ArglistField => {
                    args = self.field(pair.into_inner());
                }
                _ => unreachable!(),
            }
        }
        ctx.variables.push(Tag::Variable {
            name,
            ty,
            af,
            a,
            args,
            of_class,
            of_namespace,
        })
    }

    fn field(&self, pairs: Pairs<Rule>) -> String {
        for pair in pairs {
            if let Rule::Ident = pair.as_rule() {
                return pair.as_str().to_string();
            }
        }
        return String::new();
    }
}

#[derive(Default, Debug)]
pub struct TagContent {
    files: Vec<Tag>,
    namespaces: Vec<Tag>,
    class: Vec<Tag>,
    functions: Vec<Tag>,
    variables: Vec<Tag>,
}

impl TagContent {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub enum Tag {
    File {
        name: String,
        filename: String,
        namespace: String,
    },
    Namespace {
        name: String,
        filename: String,
        classes: Vec<String>,
    },
    Class {
        name: String,
        filename: String,
        of_namespace: Option<String>,
    },
    Function {
        name: String,
        ty: String,
        af: String,
        a: String,
        args: String,
        of_namespace: Option<String>,
        of_class: Option<String>,
    },
    Variable {
        name: String,
        ty: String,
        af: String,
        a: String,
        args: String,
        of_namespace: Option<String>,
        of_class: Option<String>,
    },
}

impl Tag {
    pub fn name(&self) -> String {
        match self {
            Tag::File { name, .. } => name.to_owned(),
            Tag::Namespace { name, .. } => name.to_owned(),
            Tag::Class { name, .. } => name.to_owned(),
            Tag::Function { name, .. } => name.to_owned(),
            Tag::Variable { name, .. } => name.to_owned(),
        }
    }
}
