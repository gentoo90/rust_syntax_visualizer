use syntex_pos::Span;
use gtk::prelude::*;
use gtk::{TreeModel, TreeIter, TreeStore, TreeModelExt, TreeStoreExtManual};

pub(crate) enum AstModelColumns {
    Type = 0,
    Kind = 1,
    HasSpan = 2,
    Lo = 3,
    Hi = 4
}

pub(crate) trait AstModelExt {
    fn get_type(&self, iter: &TreeIter) -> String;
    fn get_kind(&self, iter: &TreeIter) -> String;
    fn get_has_span(&self, iter: &TreeIter) -> bool;
    fn get_lo(&self, iter: &TreeIter) -> u32;
    fn get_hi(&self, iter: &TreeIter) -> u32;
}

macro_rules! get_ast_model_value {
    ($model:expr, $iter:expr, $col:ident, $type:ty) => {
        $model.get_value($iter, AstModelColumns::$col as i32)
                    .get::<$type>().expect("Could not get value from TreeStore")
    }
}

impl<O: IsA<TreeModel> + TreeModelExt> AstModelExt for O {
    fn get_type(&self, iter: &TreeIter) -> String {
        get_ast_model_value!(self, iter, Type, String)
    }

    fn get_kind(&self, iter: &TreeIter) -> String {
        get_ast_model_value!(self, iter, Kind, String)
    }

    fn get_has_span(&self, iter: &TreeIter) -> bool {
        get_ast_model_value!(self, iter, HasSpan, bool)
    }

    fn get_lo(&self, iter: &TreeIter) -> u32 {
        get_ast_model_value!(self, iter, Lo, u32)
    }

    fn get_hi(&self, iter: &TreeIter) -> u32 {
        get_ast_model_value!(self, iter, Hi, u32)
    }
}

pub(crate) trait AstStoreExt {
    fn insert_node(&self, iter: Option<&TreeIter>, ty: &str, kind: &str, span: Option<Span>) -> TreeIter;
}

impl<O: IsA<TreeStore> + TreeStoreExtManual> AstStoreExt for O {
    fn insert_node(&self, iter: Option<&TreeIter>, ty: &str, kind: &str, span: Option<Span>) -> TreeIter {
        let mut cols: Vec<u32> = vec![
            AstModelColumns::Type as u32,
            AstModelColumns::Kind as u32,
            AstModelColumns::HasSpan as u32
        ];
        let mut vals: Vec<&ToValue> = vec![
            &ty,
            &kind,
            &false
        ];

        if let Some(ref span) = span {
            cols.push(AstModelColumns::Lo as u32);
            cols.push(AstModelColumns::Hi as u32);

            vals[AstModelColumns::HasSpan as usize] = &true;
            vals.push(&span.lo.0);
            vals.push(&span.hi.0);
        }

        self.insert_with_values(iter, None, &cols, &vals)
    }
}
