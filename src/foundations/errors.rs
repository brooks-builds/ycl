use thiserror::Error;

#[derive(Error, Debug)]
pub enum BBError {
    #[error("Error building course nav item article: {0}")]
    CourseNavItemArticleBuilder(&'static str),
}
