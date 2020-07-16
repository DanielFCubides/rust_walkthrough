pub fn flatten<I: Iterator>(iter: I) -> Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter)
}

pub struct Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    outer: I,
    inner: Option<<I::Item as IntoIterator>::IntoIter>,
}

impl<I> Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    fn new(iter: I) -> Self {
        Self {
            outer: iter,
            inner: None,
        }
    }
}

impl<I> Iterator for Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    type Item = <I::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner{
                if let Some(item) = inner_iter.next(){
                    return Some(item)
                }
                self.inner = None
            }
            let next_inner_item = self.outer.next()?.into_iter();
            self.inner = Some(next_inner_item)
        }
        // self.outer.next()?.into_iter().next()
    }
}


// #[cfg(test)]
mod test_super{
    use super::*;
    #[test]
    // fn test_one(){
    //     let v = vec![vec![1,2,3], vec![4,5]];
    //     // let f = flatten(v.into_iter());
    //     assert_eq!(5,f.count())
    // }
    #[test]
    fn test_two(){
        let v = vec![vec![1,2], vec![4,5]];
        // let f = flatten(v.into_iter());
        let f = v.into_iter().my_flatten().filter(|v|v%2 ==0);
        assert_eq!(2,f.count())
    }
}
    
// fn name() {
//     use super::*;

//     #[test]
//     fn test_one(){
//         let v = vec![vec![1]];
//     }
// }


pub trait FlattenAdapter: Iterator + Sized{
    fn my_flatten(self) -> Flatten<Self> where <Self as Iterator>::Item: IntoIterator{
        Flatten::new(self)
    }
}


impl<I: Iterator + Sized> FlattenAdapter for I  {   }