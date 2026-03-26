use crate::utils::thermal_distribution_atom_from_index;
use bincode_trait_derive::{Decode, Encode};
use derive_more::{From, Into};
use linnet::half_edge::involution::EdgeIndex;
use serde::{Deserialize, Serialize};
use symbolica::atom::Atom;
use symbolica::parse;
use typed_index_collections::TiVec;

#[derive(From, Into, Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct ThermalNumeratorID(usize);
pub type ThermalNumeratorCollection = TiVec<ThermalNumeratorID, ThermalNumerator>;
pub type ThermalNumeratorCache<T> = TiVec<ThermalNumeratorID, T>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ThermalNumerator {
    pub positive_energies: Vec<EdgeIndex>,
    pub negative_energies: Vec<EdgeIndex>,
}

impl ThermalNumerator {
    pub(crate) fn to_atom(&self, _cut_edges: &[EdgeIndex]) -> Atom {
        let product = |positive_sign_is_negative: bool, negative_sign_is_negative: bool| {
            let positive_part = self
                .positive_energies
                .iter()
                .fold(Atom::num(1), |acc, &edge| {
                    acc * thermal_distribution_atom_from_index(
                        edge,
                        positive_sign_is_negative,
                    )
                });

            let negative_part = self
                .negative_energies
                .iter()
                .fold(Atom::num(1), |acc, &edge| {
                    acc * thermal_distribution_atom_from_index(
                        edge,
                        negative_sign_is_negative,
                    )
                });

            positive_part * negative_part
        };

        product(false, true) - product(true, false)
    }
}

impl From<ThermalNumeratorID> for Atom {
    fn from(id: ThermalNumeratorID) -> Self {
        parse!(&format!("Tnum({})", Into::<usize>::into(id.0)))
    }
}
