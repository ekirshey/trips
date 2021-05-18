use crate::object::Object;

pub struct Scene<'a> {
    pub objects: Vec<&'a Object>
}