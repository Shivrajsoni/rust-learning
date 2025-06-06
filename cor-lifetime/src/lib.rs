
#[warn(dead_code)]
pub struct StrSplit<'haystack,D> {
    remainder: Option<&'haystack str>,
    delimiter: D ,
}

pub trait Delimiter  {
    fn find_next(&self,s:&str)->Option<(usize,usize)>;
}


impl<'haystack,D> StrSplit<'haystack,D> {
    pub fn new(haystack: &'haystack str, delimiter: D ) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack,D> Iterator for StrSplit<'haystack,D> where D:Delimiter {
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        // if let Some(ref mut remainder) = self.remainder {
        //     if let Some(next_delim) = remainder.find(self.delimiter) {
        //         let until_delim = &remainder[..next_delim];
                // *remainder = &remainder[(next_delim + self.delimiter.len())..];
        //         Some(until_delim)
        //     }else {
        //         self.remainder.take()
        //     }
        // }
        // else {
        //     None
        // }

        let remainder = self.remainder.as_mut()?;
        if let Some((delim_start,delim_end)) = self.delimiter.find_next(remainder) {
            let until_delimiter = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delimiter)
        }
        else {
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self,s:&str)->Option<(usize,usize)> {
        s.find(self).map(|start| (start,start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self,s:&str)->Option<(usize,usize)> {
        s.char_indices()
        .find(|(_,c)| c == self)
        .map(|(start,_)| (start,start+1))
    }
}

pub fn until_char(s:& str,c: char)->&'_ str {
    StrSplit::new(s,c)
    .next()
    .expect("Strsplit always give you atleast one Result")

}

#[test]
fn until_char_test(){
    assert_eq!(until_char("hello world",'o'),"hell");
}



#[test]
fn it_works() {
    let haystack = "a b c d e f";
    let letters:Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e", "f"]);
}
#[test]
fn test_case() {
    let haystack = "a b c d ";
    let letters:Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
