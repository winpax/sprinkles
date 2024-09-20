use serde::Deserializer;

use super::SingleOrArray;

pub(super) fn deserialize_hash<'de, D: Deserializer<'de>>(
    data: D,
) -> Result<Option<SingleOrArray<crate::hash::Hash>>, D::Error> {
    todo!()
}
