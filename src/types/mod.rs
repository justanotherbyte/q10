mod traits;

use std::collections::HashMap;

use traits::QType;

pub struct QClass<'a> {
    attrs: HashMap<&'a str, Box<dyn QType>>,
}
