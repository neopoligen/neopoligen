// TODO: Possibly deprecate this is it if isn't used
// for making the error pages. I don't think it will
// be so you can kill this next time you see it unless
// you're still thinking about it.
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PageError {}
