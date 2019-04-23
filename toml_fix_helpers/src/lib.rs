use serde::{Serialize,Deserialize};
use std::convert::TryFrom;

#[derive(Serialize,Deserialize)]
pub struct EnumVariant{pub variant: String, pub content: String}

pub trait EnumTomlFixed where Self : TryFrom<EnumVariant>, Self: Into<EnumVariant> {

}

impl <A>  EnumTomlFixed for A where Self: TryFrom<EnumVariant>, Self: Into<EnumVariant> {

}

