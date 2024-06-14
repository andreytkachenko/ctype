use core::fmt;
use std::ops::{self};

#[derive(Debug, Clone, Copy)]
pub struct CVal<T, G> {
    pub max: T,
    _g: G,
}
impl<T: ops::Add<Output = T> + PartialEq + From<u32>, G> CVal<T, G> {
    // pub fn slice<R: RangeBounds<T>>(&self, arg: R) -> CValIter<T, G> {
    //     todo!()
    // }

    pub fn is_last(&self, i: CIndex<T, G>) -> bool {
        i.val + T::from(1u32) == self.max
    }
}

pub struct CValIter<T, G> {
    cval: CVal<T, G>,
    current: T,
}

impl<T: Copy + PartialEq + ops::AddAssign + From<u32>, G: Copy> Iterator for CValIter<T, G> {
    type Item = CIndex<T, G>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.cval.max {
            None
        } else {
            let val = self.current;
            self.current += T::from(1u32);
            Some(CIndex {
                val,
                _g: self.cval._g,
            })
        }
    }
}

impl<T: Copy + Default + PartialEq + ops::AddAssign + From<u32>, G: Copy> IntoIterator
    for CVal<T, G>
{
    type Item = CIndex<T, G>;
    type IntoIter = CValIter<T, G>;

    fn into_iter(self) -> Self::IntoIter {
        CValIter {
            cval: self,
            current: T::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct CIndex<T, G> {
    val: T,
    _g: G,
}

impl<T, G> ops::Deref for CIndex<T, G> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<T: Copy, G> CIndex<T, G> {
    #[inline]
    pub fn get(&self) -> T {
        self.val
    }
}

#[inline]
pub fn cval<T: Default, G>(max: T, _g: G) -> CVal<T, G> {
    CVal { max, _g }
}

pub struct Array2<G1, G2> {
    data: Box<[f32]>,
    pub n_rows: CVal<u32, G1>,
    pub n_cols: CVal<u32, G2>,
}

impl<G1, G2> Array2<G1, G2> {
    pub fn new_zero(n_rows: CVal<u32, G1>, n_cols: CVal<u32, G2>) -> Self {
        Self {
            data: vec![0.0; n_rows.max as usize * n_cols.max as usize].into_boxed_slice(),
            n_cols,
            n_rows,
        }
    }
}

impl<G1, G2> ops::Index<(CIndex<u32, G1>, CIndex<u32, G2>)> for Array2<G1, G2> {
    type Output = f32;

    #[inline]
    fn index(&self, index: (CIndex<u32, G1>, CIndex<u32, G2>)) -> &Self::Output {
        unsafe {
            self.data.get_unchecked(
                index.0.get() as usize * self.n_cols.max as usize + index.1.get() as usize,
            )
        }
    }
}

impl<G1, G2> ops::IndexMut<(CIndex<u32, G1>, CIndex<u32, G2>)> for Array2<G1, G2> {
    #[inline]
    fn index_mut(&mut self, index: (CIndex<u32, G1>, CIndex<u32, G2>)) -> &mut Self::Output {
        unsafe {
            self.data.get_unchecked_mut(
                index.0.get() as usize * self.n_cols.max as usize + index.1.get() as usize,
            )
        }
    }
}

impl<G1, G2> Array2<G1, G2> {
    pub fn new(n_rows: CVal<u32, G1>, n_cols: CVal<u32, G2>) -> Self {
        Self {
            data: vec![0.0; n_rows.max as usize * n_cols.max as usize].into_boxed_slice(),
            n_cols,
            n_rows,
        }
    }

    #[inline]
    pub fn n_rows(&self) -> u32 {
        self.n_rows.max
    }

    #[inline]
    pub fn n_cols(&self) -> u32 {
        self.n_cols.max
    }
}

impl<G1: Copy, G2: Copy> fmt::Display for Array2<G1, G2> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in self.n_rows {
            for j in self.n_cols {
                if j.get() > 20 {
                    write!(f, ", ...")?;
                    break;
                }

                if j.get() == 0 {
                    write!(f, "[{:0.2}", self[(i, j)])?;
                } else {
                    write!(f, ", {:0.2}", self[(i, j)])?;
                }
            }
            write!(f, "]")?;

            if !self.n_rows.is_last(i) {
                write!(f, ",\n ")?;
            }
        }
        writeln!(f, "]")?;

        Ok(())
    }
}

fn main() {
    let batch_size = cval(8u32, || ());
    let n_input = cval(16u32, || ());
    let n_output = cval(18u32, || ());

    let mut input = Array2::new(batch_size, n_input);
    let mut w1 = Array2::new(n_input, n_output);

    for i in input.n_rows {
        for j in input.n_cols {
            input[(i, j)] = random();
        }
    }

    for i in w1.n_rows {
        for j in w1.n_cols {
            w1[(i, j)] = random();
        }
    }

    let output = multiply(input, w1);

    println!("{}", output);
}

pub fn multiply<BG: Copy, IG: Copy, OG: Copy>(
    input: Array2<BG, IG>,
    w: Array2<IG, OG>,
) -> Array2<BG, OG> {
    let mut output = Array2::new(input.n_rows, w.n_cols);

    for b in output.n_rows {
        for i in output.n_cols {
            for j in input.n_cols {
                output[(b, i)] = w[(j, i)] * input[(b, j)];
            }
        }
    }

    output
}

fn random() -> f32 {
    0.0
}
