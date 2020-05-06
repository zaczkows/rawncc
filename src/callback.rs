use crate::varcontext::VarContext;
use crate::fncontext::FnContext;

pub struct Callback<'a> {
    pub var: Option<&'a mut dyn FnMut(VarContext)>,
    pub fun: Option<&'a mut dyn FnMut(FnContext)>,
}

pub trait TCallback<'a, T> {
    fn new(f: &'a mut dyn FnMut(T)) -> Self;
}

impl<'a> TCallback<'a, VarContext> for Callback<'a> {
    fn new(f: &'a mut dyn FnMut(VarContext)) -> Self {
        Callback {
            var: Some(f),
            fun: None,
        }
    }
}

impl<'a> TCallback<'a, FnContext> for Callback<'a> {
    fn new(f: &'a mut dyn FnMut(FnContext)) -> Self {
        Callback {
            var: None,
            fun: Some(f),
        }
    }
}
