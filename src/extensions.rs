pub trait OptionVecExtensions<T> {
    fn push_or_create(&mut self, value: T);
    fn push_or_create_and_get_mut(&mut self, value: T) -> &mut T;
    fn push_or_create_and_get(&mut self, value: T) -> &T;
}
impl<T> OptionVecExtensions<T> for Option<Vec<T>> {
    fn push_or_create(&mut self, value: T) {
        if let Some(children) = self {
            children.push(value);
        } else {
            *self = Some(vec![value]);
        }
    }
    fn push_or_create_and_get_mut(&mut self, value: T) -> &mut T {
        self.push_or_create(value);
        let vec = self.as_mut().expect("We just created it");
        let i = vec.len();
        vec.get_mut(i).expect("This can not be empty")
    }
    fn push_or_create_and_get(&mut self, value: T) -> &T {
        self.push_or_create(value);
        let vec = self.as_ref().expect("We just created it");
        vec.get(vec.len()).expect("This can not be empty")
    }
}
