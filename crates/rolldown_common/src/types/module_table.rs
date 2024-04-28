use crate::{ExternalModule, ExternalModuleId, NormalModule, NormalModuleId};
use index_vec::IndexVec;

pub type NormalModuleVec = IndexVec<NormalModuleId, NormalModule>;
pub type ExternalModuleVec = IndexVec<ExternalModuleId, ExternalModule>;

#[derive(Debug)]
pub struct ModuleTable {
  pub normal_modules: NormalModuleVec,
  pub external_modules: ExternalModuleVec,
}