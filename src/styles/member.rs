use std::fmt::Display;

pub enum StyleMember<D:Display>
{
    Caller(&'static str),
    Time(D)
}