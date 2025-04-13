//! General API tests.

use std::fmt::Debug;

use crate::schema::v2::{Cohort, Family, Phenopacket};

#[test]
fn top_level_elements_can_be_cloned() {
    clone_it::<Phenopacket>(None);
    clone_it::<Cohort>(None);
    clone_it::<Family>(None);
}

fn clone_it<T>(_val: Option<T>)
where
    T: Clone,
{
}

#[test]
fn top_level_elements_are_send_and_sync() {
    send_and_sync_it::<Phenopacket>(None);
    send_and_sync_it::<Cohort>(None);
    send_and_sync_it::<Family>(None);
}

fn send_and_sync_it<T>(_val: Option<T>)
where
    T: Send + Sync,
{
}

#[test]
fn top_level_elements_are_debug() {
    debug_it::<Phenopacket>(None);
    debug_it::<Cohort>(None);
    debug_it::<Family>(None);
}

fn debug_it<T>(_val: Option<T>)
where
    T: Debug,
{
}

#[test]
fn top_level_elements_are_partial_eq() {
    compare_it::<Phenopacket>(None);
    compare_it::<Cohort>(None);
    compare_it::<Family>(None);
}

fn compare_it<T>(_val: Option<T>)
where
    T: PartialEq,
{
}
