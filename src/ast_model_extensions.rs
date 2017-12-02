use syntex_pos::Span;
use gtk::prelude::*;
use gtk::{TreeModel, TreeIter, ListStore, TreeStore, TreeModelExt, ListStoreExtManual, TreeStoreExtManual};

pub(crate) enum AstModelColumns {
    Type = 0,
    Kind = 1,
    Properties = 2,
    HasSpan = 3,
    Lo = 4,
    Hi = 5,
}

pub(crate) trait AstModelExt {
    fn get_type(&self, iter: &TreeIter) -> String;
    fn get_kind(&self, iter: &TreeIter) -> String;
    fn get_properties_list(&self, iter: &TreeIter) -> ListStore;
    fn get_has_span(&self, iter: &TreeIter) -> bool;
    fn get_lo(&self, iter: &TreeIter) -> u32;
    fn get_hi(&self, iter: &TreeIter) -> u32;
    fn find_node_by_pos(&self, pos: i32) -> Option<TreeIter>;
}

macro_rules! get_ast_model_value {
    ($model:expr, $iter:expr, $col:ident, $type:ty) => {
        $model.get_value($iter, AstModelColumns::$col as i32)
                    .get::<$type>().expect("Could not get value from TreeStore")
    }
}

fn _is_in_span<T: AstModelExt + TreeModelExt>(model: &T, iter: &TreeIter, pos: u32) -> bool {
    let lo = model.get_lo(iter);
    let hi = model.get_hi(iter);

    let res = (lo <= pos) && (pos <= hi);
    println!("{} in {}:{} = {}", pos, lo, hi, res);
    res
}

fn _find_node_by_pos<T: AstModelExt + TreeModelExt>(model: &T, iter: &TreeIter, pos: i32) -> Option<TreeIter> {
    let len = model.iter_n_children(iter);
    if len == 0 && _is_in_span(model, iter, pos as u32) {
        return Some(iter.clone());
    }

    // foreach child call _find_node_by_pos untill it returns Some()
    for i in 0..len {
        let child = model.iter_nth_child(iter, i).expect("COuld not get iter child");
        let child_res = _find_node_by_pos(model, &child, pos);
        if let Some(_) = child_res {
            return child_res;
        }
    }

    None
}

impl<O: IsA<TreeModel> + TreeModelExt> AstModelExt for O {
    fn get_type(&self, iter: &TreeIter) -> String {
        get_ast_model_value!(self, iter, Type, String)
    }

    fn get_kind(&self, iter: &TreeIter) -> String {
        get_ast_model_value!(self, iter, Kind, String)
    }

    fn get_properties_list(&self, iter: &TreeIter) -> ListStore {
        get_ast_model_value!(self, iter, Properties, ListStore)
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

    fn find_node_by_pos(&self, pos: i32) -> Option<TreeIter> {
        let first = self.get_iter_first()?;
        _find_node_by_pos(self, &first, pos)
    }
}

pub(crate) trait AstStoreExt {
    fn new_ast_store() -> TreeStore;
    fn insert_node(&self, iter: Option<&TreeIter>, ty: &str, kind: &str, span: Option<Span>) -> TreeIter;
}

impl<O: IsA<TreeStore> + TreeStoreExtManual> AstStoreExt for O {
    fn new_ast_store() -> TreeStore {
        TreeStore::new(&[
            String::static_type(),
            String::static_type(),
            ListStore::static_type(),
            bool::static_type(),
            u32::static_type(),
            u32::static_type()
        ])
    }

    fn insert_node(&self, iter: Option<&TreeIter>, ty: &str, kind: &str, span: Option<Span>) -> TreeIter {
        let mut cols: Vec<u32> = vec![
            AstModelColumns::Type as u32,
            AstModelColumns::Kind as u32,
            AstModelColumns::Properties as u32,
            AstModelColumns::HasSpan as u32
        ];
        let properties_store = ListStore::new_ast_properties_store();
        properties_store.insert_property("Type", &ty);
        properties_store.insert_property("Kind", &kind);

        let mut vals: Vec<&ToValue> = vec![
            &ty,
            &kind,
            &properties_store,
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

pub(crate) enum AstPropertiesColumns {
    Name = 0,
    Value = 1,
}

pub(crate) trait AstPropertiesStoreExt {
    fn new_ast_properties_store() -> ListStore;
    fn insert_property(&self, name: &str, value: &str) -> TreeIter;
}

impl<O: IsA<ListStore> + ListStoreExtManual> AstPropertiesStoreExt for O {
    fn new_ast_properties_store() -> ListStore {
        ListStore::new(&[
            String::static_type(),
            String::static_type()
        ])
    }

    fn insert_property(&self, name: &str, value: &str) -> TreeIter {
        let cols: Vec<u32> = vec![
            AstPropertiesColumns::Name as u32,
            AstPropertiesColumns::Value as u32
        ];

        let vals: Vec<&ToValue> = vec![
            &name,
            &value
        ];

        self.insert_with_values(None, &cols, &vals)
    }
}
