use serde::de::{*};
use core::fmt;

pub struct EnumVisitor<Return>(std::marker::PhantomData<Return>);

impl<'de, Return> Visitor<'de> for EnumVisitor<Return> where Return: EnumVisitable {
    type Value = Return;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("enum #ident")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Return, V::Error> where V: SeqAccess<'de> {
        let content = seq.next_element::<String>()?.ok_or_else(|| V::Error::invalid_length(0, &self))?;

        Return::visit_variant_seq(&content, &mut seq)
    }

    fn visit_map<V>(self, mut map: V) -> Result<Return, V::Error> where V: MapAccess<'de>  {

        unimplemented!()
    }
}

pub trait EnumVisitable where Self : Sized{
    fn visit_variant_seq<'de,V>(variant: &str, seq: &mut V) -> Result<Self,V::Error> where V: SeqAccess<'de> ;

    fn visit_variant_map<'de,V>(map: &mut V) -> Result<Self,V::Error> where V: MapAccess<'de>;
}
