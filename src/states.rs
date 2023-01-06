use std::error::Error;

#[derive(Debug)]
pub struct Draft {}
#[derive(Debug)]
pub struct AwaitingReview {}
#[derive(Debug)]
pub struct Posted {}

pub trait State
where
    Self: std::fmt::Debug,
{
    fn add_text(&self) -> Result<(), Box<dyn Error>> {
        Err(format!("Expected Post in Draft, found state {:?}", self).into())
    }

    fn request_review(&self) -> Result<(), Box<dyn Error>> {
        Err(format!("Expected Post in Draft, found state {:?}", self).into())
    }

    fn approve(&self) -> Result<(), Box<dyn Error>> {
        Err(format!("Expected Post in AwaitingReview, found state {:?}", self).into())
    }

    fn content(&self) -> Result<(), Box<dyn Error>> {
        Err(format!("Expected Post in Posted, found state {:?}", self).into())
    }
}

impl State for Draft {
    fn add_text(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn request_review(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl State for AwaitingReview {
    fn approve(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl State for Posted {
    fn content(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
