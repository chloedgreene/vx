use crate::attributes::{attribute_info, self};

pub struct method_info{
    pub access_flag:u16,
    pub name_index:u16,
    pub descriptor_index:u16,
    pub attributes_count: u16,
    pub attributes: Vec<attributes::attribute>
}