use crate::db::model::{bucketlist_item::BucketlistItem, destination::Destination};
use serde::Serialize;

#[derive(Serialize)]
pub struct BucketlistItemWithDestinationDTO {
  #[serde(flatten)]
  pub bucketlist_item: BucketlistItem,
  pub destination: Destination
}
