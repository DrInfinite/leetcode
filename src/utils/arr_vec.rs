pub trait ToNestedVec {
    type Output;

    fn to_nested_vec(self) -> Self::Output;
}

impl ToNestedVec for i32 {
    type Output = i32;

    fn to_nested_vec(self) -> Self::Output {
        self
    }
}

impl<T, const N: usize> ToNestedVec for [T; N]
where
    T: ToNestedVec,
{
    type Output = Vec<T::Output>;

    fn to_nested_vec(self) -> Self::Output {
        self.into_iter().map(|item| item.to_nested_vec()).collect()
    }
}
