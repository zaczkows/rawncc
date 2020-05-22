use crate::cast_context::CastContext;
use crate::fn_context::FnContext;
use crate::varcontext::VarContext;

pub struct Callback<'a> {
    pub var: Option<&'a mut dyn FnMut(VarContext)>,
    pub fun: Option<&'a mut dyn FnMut(FnContext)>,
    pub cast: Option<&'a mut dyn FnMut(CastContext)>,
}

pub trait TCallback<'a, T> {
    fn new(f: &'a mut dyn FnMut(T)) -> Self;
}

impl<'a> TCallback<'a, VarContext> for Callback<'a> {
    fn new(f: &'a mut dyn FnMut(VarContext)) -> Self {
        Callback {
            var: Some(f),
            fun: None,
            cast: None,
        }
    }
}

impl<'a> TCallback<'a, FnContext> for Callback<'a> {
    fn new(f: &'a mut dyn FnMut(FnContext)) -> Self {
        Callback {
            var: None,
            fun: Some(f),
            cast: None,
        }
    }
}

impl<'a> TCallback<'a, CastContext> for Callback<'a> {
    fn new(f: &'a mut dyn FnMut(CastContext)) -> Self {
        Callback {
            var: None,
            fun: None,
            cast: Some(f),
        }
    }
}
