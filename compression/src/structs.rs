pub struct Heap<'a> {
    pub val: i32,
    pub left: Option<&'a Heap<'a>>,
    pub right: Option<&'a Heap<'a>>
}

impl<'a> Heap<'a> {
    pub fn new(val: i32, left: Option<&'a Heap<'a>>,
               right: Option<&'a Heap<'a>>) -> Heap<'a> {
        Heap {val: val, left: left, right: right}
    }

}
