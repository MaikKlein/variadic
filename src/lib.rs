pub trait Variadic<T, A = T, B = A, C = B> {
    type OneLess: Variadic<A, B, C>;
    fn pop(self) -> (Option<T>, Self::OneLess);
}

pub struct VarArgs4<A, B, C, D>(pub A, pub B, pub C, pub D);
pub struct VarArgs3<A, B, C>(pub A, pub B, pub C);
pub struct VarArgs2<A, B>(pub A, pub B);
pub struct VarArgs1<A>(pub A);
pub struct VarArgs0<A> {
    _m: std::marker::PhantomData<A>,
}

impl<T, A, B, C> Variadic<T, A, B, C> for VarArgs4<T, A, B, C> {
    type OneLess = VarArgs3<A, B, C>;
    fn pop(self) -> (Option<T>, Self::OneLess) {
        let VarArgs4(t, a, b, c) = self;
        (Some(t), VarArgs3(a, b, c))
    }
}

impl<T, A, B> Variadic<T, A, B> for VarArgs3<T, A, B> {
    type OneLess = VarArgs2<A, B>;
    fn pop(self) -> (Option<T>, Self::OneLess) {
        let VarArgs3(t, a, b) = self;
        (Some(t), VarArgs2(a, b))
    }
}

impl<T, A> Variadic<T, A> for VarArgs2<T, A> {
    type OneLess = VarArgs1<A>;
    fn pop(self) -> (Option<T>, Self::OneLess) {
        let VarArgs2(t, a) = self;
        (Some(t), VarArgs1(a))
    }
}

impl<T> Variadic<T> for VarArgs1<T> {
    type OneLess = VarArgs0<T>;
    fn pop(self) -> (Option<T>, Self::OneLess) {
        let VarArgs1(t) = self;
        (Some(t), VarArgs0 { _m: std::marker::PhantomData })
    }
}

impl<T> Variadic<T> for VarArgs0<T> {
    type OneLess = VarArgs0<T>;
    fn pop(self) -> (Option<T>, Self) {
        (None, VarArgs0 { _m: std::marker::PhantomData })
    }
}

