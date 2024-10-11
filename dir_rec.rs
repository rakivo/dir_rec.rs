use std::fs::read_dir;
use std::path::PathBuf;
use std::collections::VecDeque;

pub struct DirRec {
    stack: VecDeque::<PathBuf>
}

impl DirRec {
    #[inline(always)]
    pub fn new<P: Into::<PathBuf>>(root: P) -> DirRec {
        DirRec {stack: vec![root.into()].into()}
    }
}

impl Iterator for DirRec {
    type Item = PathBuf;

    fn next(&mut self) -> Option::<Self::Item> {
        while let Some(p) = self.stack.pop_front() {
            if p.is_file() { return Some(p) }
            match read_dir(&p) {
                Ok(es) => es.filter_map(Result::ok).for_each(|e| {
                    self.stack.push_back(e.path())
                }),
                Err(e) => eprintln!("ERROR: {e}")
            }
        } None
    }
}
