use super::{Matrix, Eig, Triangular};
use lapack::*;
use operations::*;
use matrixerror::MatrixError;




// get the eigenvalues of a matrix.
pub fn eigenvalues(a : &mut Matrix<f64>, eorv : Eig, tri : Triangular) -> Result<Matrix<f64>,MatrixError>{
    let n = a.row_size;
    let mut w = vec![0.0; n];
    let mut work = vec![0.0; 4 * n];
    let lwork = 4 * n as isize;
    let mut info = 0;
    let e = match eorv {
        Eig::Eigenvalues => b'V',
        Eig::EigenvaluesAndEigenvectors => b'E'
    };
    let t = match tri {
        Triangular::Upper => b'U',
        Triangular::Lower => b'L'
    };
    dsyev(e, t, n, &mut a.elements, n, &mut w, &mut work, lwork, &mut info);
    match info {
        1 => Err(MatrixError::LapackComputationError),
        0 => Ok (Matrix {
            elements : w.to_owned(),
            row_size : w.len(),
            col_size : 1,
            transpose : false,
        }),
        -1 => Err(MatrixError::LapackInputError),
        _ => Err(MatrixError::UnknownError)
    }

}







pub fn singular_values(a : &mut Matrix<f64>) -> Result<Matrix<f64>, MatrixError> {
        let mut at =  a.transpose();
        let adjoint_operator = dot(a,&mut at);
        let e = eigenvalues(&mut adjoint_operator.unwrap(), Eig :: Eigenvalues, Triangular::Upper);
         match matrix_map(&|x : &f64| x.sqrt(), &mut e.unwrap()) {
                Ok(mat) => Ok(mat),
                Err(mat) => Err(mat),
         }

}