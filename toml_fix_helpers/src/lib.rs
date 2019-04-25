use serde::{Serialize,Deserialize};
use std::convert::{TryFrom,TryInto};

#[derive(Serialize,Deserialize)]
pub struct EnumVariant{pub variant: String, pub content: String}

pub trait EnumTomlFixed where Self : TryFrom<EnumVariant>, Self: TryInto<EnumVariant> {

}

impl <A>  EnumTomlFixed for A where Self: TryFrom<EnumVariant>, Self: TryInto<EnumVariant> {

}

