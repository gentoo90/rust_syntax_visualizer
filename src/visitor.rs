use syntex_syntax::ast::*;
use syntex_syntax::visit::*;
use syntex_pos::Span;
use syntex_syntax::print::pprust;
use gtk::prelude::*;
use gtk::{ListStore, TreeStore, TreeIter};
use ast_model_extensions::{AstModelExt, AstStoreExt, AstPropertiesStoreExt};

pub(crate) struct TreeVisitor {
    pub tree: TreeStore,
    pub iters: Vec<TreeIter>
}

macro_rules! visit {
    ($self:ident, $props:ident, ($type:expr, $kind:expr) => $walk:block) => {
        let iter = $self.tree.insert_node($self.iters.last(), $type, $kind, None);
        let $props = $self.tree.get_properties_list(&iter);
        $self.iters.push(iter);
        $walk;
        $self.iters.pop();
    };
    ($self:ident, $props:ident, ($type:expr, $kind:expr, $span:expr) => $walk:block) => {
        let iter = $self.tree.insert_node($self.iters.last(), $type, $kind, Some($span));
        let $props = $self.tree.get_properties_list(&iter);
        $self.iters.push(iter);
        $walk;
        $self.iters.pop();
    };
}

impl TreeVisitor {
    pub fn new() -> TreeVisitor {
        TreeVisitor{
            tree: TreeStore::new_ast_store(),
            iters: vec![]
        }
    }

    fn _visit_path(&mut self, path: &Path) {
        visit!(self, path_props, ("Path", "", path.span) => {
            walk_path(self, path);
        });
    }
}

impl<'ast> Visitor<'ast> for TreeVisitor {
    fn visit_name(&mut self, span: Span, name: Name) {
        let iter = self.tree.insert_node(self.iters.last(), "Name", "", Some(span));
        let props = self.tree.get_properties_list(&iter);
        props.insert_property("Name", &name.as_str());
    }

    fn visit_ident(&mut self, span: Span, ident: Ident) {
        visit!(self, props, ("Ident", "", span) => {
            props.insert_property("Name", &pprust::ident_to_string(ident));
            walk_ident(self, span, ident);
        });
    }

    fn visit_mod(&mut self, m: &'ast Mod, _span: Span, _attrs: &[Attribute], _n: NodeId) {
        visit!(self, props, ("Mod", "", m.inner)/*span*/ => {
            walk_mod(self, m);
        });
    }

    // fn visit_global_asm(&mut self, ga: &'ast GlobalAsm) { walk_global_asm(self, ga) }

    fn visit_foreign_item(&mut self, i: &'ast ForeignItem) {
        visit!(self, props, ("ForeignItem", "") => {
            walk_foreign_item(self, i);
        });
    }

    fn visit_item(&mut self, i: &'ast Item) {
        let kind = match i.node {
            ItemKind::ExternCrate(..) => "ExternCrate",
            ItemKind::Use(..) => "Use",
            ItemKind::Static(..) => "Static",
            ItemKind::Const(..) => "Const",
            ItemKind::Fn(..) => "Fn",
            ItemKind::Mod(..) => "Mod",
            ItemKind::ForeignMod(..) => "ForeignMod",
            ItemKind::GlobalAsm(..) => "GlobalAsm",
            ItemKind::Ty(..) => "Ty",
            ItemKind::Enum(..) => "Enum",
            ItemKind::Struct(..) => "Struct",
            ItemKind::Union(..) => "Union",
            ItemKind::Trait(..) => "Trait",
            ItemKind::DefaultImpl(..) => "DefaultImpl",
            ItemKind::Impl(..) => "Impl",
            ItemKind::Mac(..) => "Mac",
            ItemKind::MacroDef(..) => "MacroDef",
        };
        visit!(self, props, ("Item", kind) => {
            walk_item(self, i);
        });
    }

    fn visit_local(&mut self, l: &'ast Local) {
        visit!(self, props, ("Local", "", l.span) => {
            walk_local(self, l);
        });
    }

    fn visit_block(&mut self, b: &'ast Block) {
        visit!(self, props, ("Block", "", b.span) => {
            walk_block(self, b);
        });
    }

    fn visit_stmt(&mut self, s: &'ast Stmt) {
        let kind = match s.node {
            StmtKind::Local(..) => "Local",
            StmtKind::Item(..) => "Item",
            StmtKind::Expr(..) => "Expr",
            StmtKind::Semi(..) => "Semi",
            StmtKind::Mac(..) => "Mac",
        };
        visit!(self, props, ("Stmt", kind, s.span) => {
            walk_stmt(self, s);
        });
    }

    fn visit_arm(&mut self, a: &'ast Arm) {
        visit!(self, props, ("Arm", "") => {
            walk_arm(self, a);
        });
    }

    fn visit_pat(&mut self, p: &'ast Pat) {
        let kind = match p.node {
            PatKind::Wild => "Wild",
            PatKind::Ident(..) => "Ident",
            PatKind::Struct(..) => "Struct",
            PatKind::TupleStruct(..) => "TupleStruct",
            PatKind::Path(..) => "Path",
            PatKind::Tuple(..) => "Tuple",
            PatKind::Box(..) => "Box",
            PatKind::Ref(..) => "Ref",
            PatKind::Lit(..) => "Lit",
            PatKind::Range(..) => "Range",
            PatKind::Slice(..) => "Slice",
            PatKind::Mac(..) => "Mac",
        };
        visit!(self, props, ("Pat", kind, p.span) => {
            walk_pat(self, p);
        });
    }

    fn visit_expr(&mut self, ex: &'ast Expr) {
        let kind = match ex.node {
            ExprKind::Box(..) => "Box",
            ExprKind::InPlace(..) => "InPlace",
            ExprKind::Array(..) => "Array",
            ExprKind::Call(..) => "Call",
            ExprKind::MethodCall(..) => "MethodCall",
            ExprKind::Tup(..) => "Tup",
            ExprKind::Binary(..) => "Binary",
            ExprKind::Unary(..) => "Unary",
            ExprKind::Lit(..) => "Lit",
            ExprKind::Cast(..) => "Cast",
            ExprKind::Type(..) => "Type",
            ExprKind::If(..) => "If",
            ExprKind::IfLet(..) => "IfLet",
            ExprKind::While(..) => "While",
            ExprKind::WhileLet(..) => "WhileLet",
            ExprKind::ForLoop(..) => "ForLoop",
            ExprKind::Loop(..) => "Loop",
            ExprKind::Match(..) => "Match",
            ExprKind::Closure(..) => "Closure",
            ExprKind::Block(..) => "Block",
            ExprKind::Catch(..) => "Catch",
            ExprKind::Assign(..) => "Assign",
            ExprKind::AssignOp(..) => "AssignOp",
            ExprKind::Field(..) => "Field",
            ExprKind::TupField(..) => "TupField",
            ExprKind::Index(..) => "Index",
            ExprKind::Range(..) => "Range",
            ExprKind::Path(..) => "Path",
            ExprKind::AddrOf(..) => "AddrOf",
            ExprKind::Break(..) => "Break",
            ExprKind::Continue(..) => "Continue",
            ExprKind::Ret(..) => "Ret",
            ExprKind::InlineAsm(..) => "InlineAsm",
            ExprKind::Mac(..) => "Mac",
            ExprKind::Struct(..) => "Struct",
            ExprKind::Repeat(..) => "Repeat",
            ExprKind::Paren(..) => "Paren",
            ExprKind::Try(..) => "Try",
        };
        visit!(self, props, ("Expr", kind, ex.span) => {
            walk_expr(self, ex);
        });
    }

    // fn visit_expr_post(&mut self, _ex: &'ast Expr) {

    // }

    fn visit_ty(&mut self, t: &'ast Ty) {
        let kind = match t.node {
            TyKind::Slice(..) => "Slice",
            TyKind::Array(..) => "Array",
            TyKind::Ptr(..) => "Ptr",
            TyKind::Rptr(..) => "Rptr",
            TyKind::BareFn(..) => "BareFn",
            TyKind::Never => "Never",
            TyKind::Tup(..) => "Tup",
            TyKind::Path(..) => "Path",
            TyKind::TraitObject(..) => "TraitObject",
            TyKind::ImplTrait(..) => "ImplTrait",
            TyKind::Paren(..) => "Paren",
            TyKind::Typeof(..) => "Typeof",
            TyKind::Infer => "Infer",
            TyKind::ImplicitSelf => "ImplicitSelf",
            TyKind::Mac(..) => "Mac",
            TyKind::Err => "Err",
        };
        visit!(self, props, ("Ty", kind, t.span) => {
            walk_ty(self, t);
        });
    }

    fn visit_generics(&mut self, g: &'ast Generics) {
        visit!(self, props, ("Generics", "", g.span) => {
            walk_generics(self, g);
        });
    }

    fn visit_where_predicate(&mut self, p: &'ast WherePredicate) {
        visit!(self, props, ("WherePredicate", "") => {
            walk_where_predicate(self, p);
        });
    }

    fn visit_fn(&mut self, fk: FnKind<'ast>, fd: &'ast FnDecl, s: Span, _: NodeId) {
        let kind = match fk {
            FnKind::ItemFn(..) => "ItemFn",
            FnKind::Method(..) => "Method",
            FnKind::Closure(..) => "Closure",
        };
        visit!(self, props, ("Fn", kind, s) => {
            walk_fn(self, fk, fd, s);
        });
    }

    fn visit_trait_item(&mut self, ti: &'ast TraitItem) {
        visit!(self, props, ("TraitItem", "", ti.span) => {
            walk_trait_item(self, ti);
        });
    }

    fn visit_impl_item(&mut self, ii: &'ast ImplItem) {
        visit!(self, props, ("ImplItem", "", ii.span) => {
            walk_impl_item(self, ii);
        });
    }

    fn visit_trait_ref(&mut self, t: &'ast TraitRef) {
        visit!(self, props, ("TraitRef", "") => {
            walk_trait_ref(self, t);
        });
    }

    fn visit_ty_param_bound(&mut self, bounds: &'ast TyParamBound) {
        visit!(self, props, ("TyParamBound", "") => {
            walk_ty_param_bound(self, bounds);
        });
    }

    fn visit_poly_trait_ref(&mut self, t: &'ast PolyTraitRef, m: &'ast TraitBoundModifier) {
        visit!(self, props, ("PolyTraitRef", "") => {
            walk_poly_trait_ref(self, t, m);
        });
    }

    fn visit_variant_data(&mut self, s: &'ast VariantData, _: Ident,
                          _: &'ast Generics, _: NodeId, span: Span) {
        visit!(self, props, ("VariantData", "", span) => {
            walk_struct_def(self, s);
        });
    }

    fn visit_struct_field(&mut self, s: &'ast StructField) {
        visit!(self, props, ("StructField", "", s.span) => {walk_struct_field(self, s)});
    }

    fn visit_enum_def(&mut self, enum_definition: &'ast EnumDef,
                      generics: &'ast Generics, item_id: NodeId, span: Span) {
        visit!(self, props, ("EnumDef", "", span) => {
            walk_enum_def(self, enum_definition, generics, item_id);
        });
    }

    fn visit_variant(&mut self, v: &'ast Variant, g: &'ast Generics, item_id: NodeId) {
        visit!(self, props, ("Variant", "") => {
            walk_variant(self, v, g, item_id);
        });
    }

    fn visit_lifetime(&mut self, lifetime: &'ast Lifetime) {
        visit!(self, props, ("Lifetime", "", lifetime.span) => {
            walk_lifetime(self, lifetime);
        });
    }

    fn visit_lifetime_def(&mut self, lifetime: &'ast LifetimeDef) {
        visit!(self, props, ("LifetimeDef", "") => {
            walk_lifetime_def(self, lifetime);
        });
    }

    fn visit_mac(&mut self, _mac: &'ast Mac) {
        // visit::walk_mac(self, _mac)
    }

    fn visit_mac_def(&mut self, _mac: &'ast MacroDef, _id: NodeId) {
        // Nothing to do
    }

    fn visit_path(&mut self, path: &'ast Path, _id: NodeId) {
        self._visit_path(path);
    }

    fn visit_path_list_item(&mut self, prefix: &'ast Path, item: &'ast PathListItem) {
        visit!(self, props, ("ListItem", "") => {
            walk_path_list_item(self, prefix, item);
        });
    }

    fn visit_path_segment(&mut self, path_span: Span, path_segment: &'ast PathSegment) {
        visit!(self, props, ("PathSegment", "", path_segment.span) => {
            walk_path_segment(self, path_span, path_segment);
        });
    }

    fn visit_path_parameters(&mut self, path_span: Span, path_parameters: &'ast PathParameters) {
        //FIXME: add span if match Parenthesized()
        visit!(self, props, ("PathParameters", "", path_span) => {
            walk_path_parameters(self, path_span, path_parameters);
        });
    }

    fn visit_assoc_type_binding(&mut self, type_binding: &'ast TypeBinding) {
        visit!(self, props, ("TypeBinding", "") => {
            walk_assoc_type_binding(self, type_binding);
        });
    }

    fn visit_attribute(&mut self, attr: &'ast Attribute) {
        visit!(self, attr_props, ("Attribute", "", attr.span) => {
            self._visit_path(&attr.path);
        });
    }

    fn visit_vis(&mut self, vis: &'ast Visibility) {
        let kind = match *vis {
            Visibility::Crate(_span) => "Crate",
            Visibility::Inherited => "Inherited",
            Visibility::Public => "Public",
            Visibility::Restricted{..} => "Restricted"
        };
        visit!(self, props, ("Vis", kind) => {
            walk_vis(self, vis);
        });
    }

    fn visit_fn_ret_ty(&mut self, ret_ty: &'ast FunctionRetTy) {
        visit!(self, props, ("FnRetTy", "", ret_ty.span()) => {
            walk_fn_ret_ty(self, ret_ty);
        });
    }
}
