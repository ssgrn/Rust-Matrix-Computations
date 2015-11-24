// rcond, rank,
//     det, invlndet,


use super::{Matrix,Norm, Condition};

use matrixerror::MatrixError;
use lapack::*;
use factorizations::*;


/// Calculate the norm of a matrix (1-Norm, Infinity-Norm, Frobenius Norm, or Max Absolute Value)
pub fn norm(a : &Matrix<f64>, inorm : Norm) -> f64 {
    let norm = match inorm {
         Norm :: OneNorm => b'1',
         Norm :: InfinityNorm => b'I',
         Norm :: FrobeniusNorm => b'F',
         Norm :: MaxAbsValue => b'M',
    };
    let m = a.col_size;
    let n = a.row_size;
    let lda = a.row_size;
    let mut work = vec![0.0; 4*n];
    dlange(norm, m, n, &a.elements,lda, &mut work)


}

/// Get the number of linearly independent rows or columns. TODO: use QR
pub fn rank(a : &mut Matrix<f64>) -> Result<usize, MatrixError> {

    if let Ok(mut s) = singular_values(a) {
        let z = &mut s.elements;
        println!("{:?}",z);
        return Ok(z.iter().filter(|x| *x > &0.0).collect::<Vec<&f64>>().len())
    }
    Ok(0 as usize)

}

/// Determine the condition of a matrix via the condition number.
pub fn cond(a : &mut Matrix <f64>, inorm : Norm) -> Result<Condition, MatrixError>{
    let nm = match inorm {
         Norm :: OneNorm => b'1',
         Norm :: InfinityNorm => b'I',
         Norm :: FrobeniusNorm => return Err(MatrixError::LapackInputError),
         Norm :: MaxAbsValue => return Err(MatrixError::LapackInputError)
    };
    let cond = Condition :: NotAvailable;
    if let Ok((l, ipiv)) = lufact (a){
        let n = l.col_size;
        let lda = l.row_size;
        let anorm = norm(&l, Norm::OneNorm);
        let mut work = vec![0.0; 4 * n];
        let mut iwork = vec![0;  n];
        let mut rcond = 0.0;
        let mut info = 0;
        dgecon(nm, n, &l.elements, lda, anorm, &mut rcond, &mut work, &mut iwork, &mut info);
        let cond = match rcond.recip().gt(&1000.0){
            false => Condition :: WellConditioned,
            true => {
                match rcond.recip().le(&500000.0){
                    true => Condition :: IllConditioned,
                    false => Condition :: Singular,
                }
            }

        };

        match info {
            x if x > 0 => return Err(MatrixError::LapackComputationError),
            0 => return Ok(cond),
            x if x < 0 => return  Err(MatrixError::LapackInputError),
            _ => return Err(MatrixError::UnknownError),
        };

    }
    Ok(cond)
}
