pub trait Messanger {
    fn send(&self, message: &str);
}

pub struct LimitTracker<'m, T: Messanger> {
    messanger: &'m T,
    value: usize,
    max: usize,
}

impl<'m, T> LimitTracker<'m, T> where T: Messanger {
    
}
