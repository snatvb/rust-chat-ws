use super::users;
use crate::{
    chat::store::{PeerItem, Store},
    id_record::IDControl,
};

#[inline(always)]
pub fn user_and_peer<'a>(
    store: &'a Store,
    user_id: &users::Id,
) -> Option<(&'a PeerItem, &'a users::User)> {
    store.peers.get(user_id).and_then(|(_, peer)| {
        peer.user_id
            .and_then(|id| store.users.get(&id))
            .map(|user| (peer, user))
    })
}
