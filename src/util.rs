
//https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=88023e8c33fc46a32f13214c77d5d07a
trait VecExt<T: Clone + Default> {
    fn set_index(&mut self, index:usize, value:T);
}

impl <T: Clone + Default> VecExt<T> for Vec<T> {
    fn set_index(&mut self, index:usize, value: T) {
        if self.len() <= index {
            self.resize(index+1, T::default());
        }
        
        unsafe {
            let elem = self.get_unchecked_mut(index);
            *elem = value;
        }
    }
}