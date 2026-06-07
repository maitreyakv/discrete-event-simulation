use crate::Model;

use super::EventKey;

pub(crate) struct AntiEvent<M>(EventKey<M>)
where
    M: Model;

impl<M> From<AntiEvent<M>> for EventKey<M>
where
    M: Model,
{
    fn from(value: AntiEvent<M>) -> Self {
        value.0
    }
}
