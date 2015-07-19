use std;

use error;


pub type Result<T> = std::result::Result<T, error::Error>;


// impl<T> From<error::Error> for Result<T> {
//     fn from(e: error::Error) -> Result<T> {
//         Result<T>::Err(e)
//     }
// }
