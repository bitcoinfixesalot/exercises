pub fn map<A, B, F>(input: Vec<A>, mut function: F) -> Vec<B>
where
    F: FnMut(A) -> B,
{
    let mut result = Vec::with_capacity(input.len());
    for i in input {
        result.push(function(i));
    }
    result
}
