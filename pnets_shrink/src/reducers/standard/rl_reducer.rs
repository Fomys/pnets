use std::collections::HashMap;

use pnets::standard::Net;
use pnets::{arc, PlaceId, TransitionId};

use crate::modifications::Agglomeration;
use crate::modifications::{InequalityReduction, Modification};
use crate::reducers::Reduce;

/// RL Reducer
///
/// Réduit les "feuilles" présentes sur le modèle viral epidemic
pub struct RLReducer;

impl RLReducer {
    /// Create a new RL reducer
    pub fn new() -> Self {
        Self {}
    }
}

impl Reduce<Net> for RLReducer {
    fn reduce(&self, net: &mut Net, modifications: &mut Vec<Modification>) {
        'original_transition: for tr in (0..net.transitions.len()).map(|i| TransitionId::from(i)) {
            // Recherche d'une place de type "production" pour pouvoir faire la suite des vérifications
            let produce_place = {
                // Vérification que la transition parente de la place finale est de type "transition contrainte"
                // Deux entrées, une sortie, poids de 1 partout
                let transition = &net[tr];
                if transition.consume.len() != 2 // 2 entrées
                    || transition.produce.len() != 1 // 1 sortie
                    || transition.consume.iter().any(|(_, w)| *w != 1) // Entrée/sorties unitaires
                    || transition.produce.iter().any(|(_, w)| *w != 1)
                {
                    continue 'original_transition;
                }

                // On doit avoir sur cette transition deux consommation: la place contrainte et la place production
                // - place contrainte: aucune entrée -> pas besoin de vérifier, ça sera fait plus tard
                // - place de production: une seule entrée
                match transition
                    .consume
                    .iter()
                    .map(|(pl, _)| &net[*pl])
                    .find(|place| place.produced_by.len() == 1)
                {
                    None => {
                        // Aucune des deux places en entrée ne peut être utilisée comme "place productrice"
                        continue 'original_transition;
                    }
                    Some(place) => place,
                }
            };
            let mut constraint_places = vec![];
            // Recherche des places contraintes
            // - Itération sur toutes les transitions de sortie de la produce_place
            // - Recherche de la première place consommée autre que la place production
            for transition in produce_place.consumed_by.iter().map(|(tr, _)| &net[*tr]) {
                if let Some(&(constraint_pl, w)) = transition
                    .consume
                    .iter()
                    .find(|(pl, _)| *pl != produce_place.id())
                {
                    if constraint_places.contains(&constraint_pl) && w != 1 {
                        continue 'original_transition;
                    }
                    constraint_places.push(constraint_pl);
                } else {
                    // La transition n'a pas d'entrée autre que produce_place
                    continue 'original_transition;
                }
            }

            if constraint_places.is_empty() {
                // Aucune place contrainte n'a été trouvé, donc aucune réduction possible
                continue 'original_transition;
            }

            let mut output_transitions = HashMap::<PlaceId, Vec<TransitionId>>::default();

            // Vérification que toutes les transitions des contraintes sont de type 2 entrées, 0 sortie
            // et que chaque place de sortie est de type "finale"
            for &constraint_pl in &constraint_places {
                let constraint_place = &net[constraint_pl];
                for (transition, w) in constraint_place
                    .consumed_by
                    .iter()
                    .map(|(tr, w)| (&net[*tr], *w))
                {
                    //.flat_map(|pl| net[*pl].consumed_by.iter())
                    //{
                    if w != 1 || transition.consume.len() != 2 || !transition.produce.is_empty() {
                        // L'une des transitions de sortie de la place contrainte n'est pas 2 entrées/1 sortie
                        continue 'original_transition;
                    }

                    // Extraction de la place production de cette transition
                    let (produce_pl, w) = match transition
                        .consume
                        .iter()
                        .find(|(pl, _)| *pl != constraint_pl)
                    {
                        None => continue 'original_transition,
                        Some(&(pl, w)) => (pl, w),
                    };
                    let produce_place = &net[produce_pl];
                    if w != 1 || produce_place.produced_by.len() != 1 {
                        continue 'original_transition;
                    }

                    // Ajout de la transition à la hashmap
                    match output_transitions.get_mut(&produce_pl) {
                        None => {
                            output_transitions.insert(produce_pl, vec![transition.id()]);
                        }
                        Some(vec) => vec.push(transition.id()),
                    }
                }

                if constraint_place.consumed_by.len() != output_transitions.len() {
                    continue 'original_transition;
                }
            }

            // Vérification que toutes les product_places ont la même transition d'origine
            let source_tr = net[*output_transitions.keys().next().unwrap()]
                .produced_by
                .iter()
                .next()
                .unwrap()
                .0;

            for &(tr, w) in output_transitions
                .keys()
                .map(|pl| net[*pl].produced_by.iter().next().unwrap())
            {
                if w != 1 {
                    continue 'original_transition;
                }
                if tr != source_tr {
                    continue 'original_transition;
                }
            }
            let new_pl = net.create_place();

            for &pl in &constraint_places {
                modifications.push(Modification::InequalityReduction(InequalityReduction {
                    deleted_places: vec![(pl, 1)],
                    kept_places: vec![],
                    constant: net[pl].initial as isize,
                }));
                net.delete_place(pl);
            }

            let sum: usize = constraint_places.iter().map(|pl| net[*pl].initial).sum();
            let n = output_transitions.len() as isize;

            // Si on peut faire une agglomération directement on la fait pour éviter de créer des places temporaires
            if net[source_tr].consume.len() == 1
                && net[source_tr].produce.len() == output_transitions.len()
            {
                let parent_pl = net[source_tr].consume.iter().next().unwrap().0;
                for old_arc in net[parent_pl].get_arcs() {
                    match old_arc {
                        arc::Kind::Consume(_, tr, w) => {
                            net.add_arc(arc::Kind::Consume(new_pl, tr, w)).unwrap();
                        }
                        arc::Kind::Produce(_, tr, w) => {
                            net.add_arc(arc::Kind::Produce(new_pl, tr, w)).unwrap();
                        }
                        _ => {}
                    }
                }
                net[new_pl].initial = net[parent_pl].initial;
                net.delete_place(parent_pl);
                net.delete_transition(source_tr);

                modifications.push(Modification::Agglomeration(Agglomeration {
                    deleted_places: constraint_places
                        .iter()
                        .map(|pl| (*pl, -1))
                        .chain(output_transitions.keys().map(|pl| (*pl, 1)))
                        .chain(vec![(parent_pl, n)])
                        .collect(),
                    constant: sum as isize,
                    new_place: new_pl,
                    factor: n,
                }));

                for &pl in output_transitions.keys() {
                    modifications.push(Modification::InequalityReduction(InequalityReduction {
                        deleted_places: vec![(pl, 1), (parent_pl, 1)],
                        constant: 0,
                        kept_places: vec![(new_pl, 1)],
                    }));

                    net.delete_place(pl);
                }
            } else {
                // Application de la modification
                net.add_arc(arc::Kind::Produce(new_pl, source_tr, 1))
                    .unwrap();

                modifications.push(Modification::Agglomeration(Agglomeration {
                    deleted_places: constraint_places
                        .iter()
                        .map(|pl| (*pl, -1))
                        .chain(output_transitions.keys().map(|pl| (*pl, 1)))
                        .collect(),
                    constant: sum as isize,
                    new_place: new_pl,
                    factor: n,
                }));

                // Création des places temporaires de sortie et création de la réduction
                for &pl in output_transitions.keys() {
                    modifications.push(Modification::InequalityReduction(InequalityReduction {
                        deleted_places: vec![(pl, 1)],
                        constant: 0,
                        kept_places: vec![(new_pl, 1)],
                    }));

                    net.delete_place(pl);
                }
            }
        }
    }
}
