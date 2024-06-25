#[derive(Debug, Copy, Clone)]

/// Naming: 12345 I 678910 (I BC)
/// 12345 Abbreviation of spanning tree construction
/// 5678910 Abbreviation of edge weights used
/// BC Bounded Cliques (optional)
/// I separation letter
pub enum HeuristicTypes {
    // For comparison of edge weights:
    MSTreILeDif,
    MSTreINegIn,
    MSTreIUnion,
    MSTreIDisjU,

    // For comparison of combined edge weights:
    MSTreINiTLd,
    MSTreILdTNi,

    // For comparison of spanning tree construction:
    FilWhINiTLd,
    FWhUEINiTLd, // Update edges in clique graph according to filling up whilst building spanning tree
    FWBagINonee,

    // For comparison with GreedyX
    GreedyDegreeFillIn,

    // For comparison with bounded clique
    FilWhINiTLdIBC(usize),

    // Old / Not used
    FilWhINegIn,
    FilWhILeDif,
    FiWhTINiTLd,
    FilWhILdTNi,
    MSTreINiTLdIBC(usize),
    FiWhTINiTLdIBC(usize),
}

use csv::Writer;
use petgraph::graph::NodeIndex;
use HeuristicTypes::*;

pub const TEST_SUITE: [(fn() -> Vec<HeuristicTypes>, &str); 4] = [
    (comparison_of_edge_weights, "comparison_of_edge_weights"),
    (
        comparison_of_combined_edge_weights,
        "comparison_of_combined_edge_weights",
    ),
    (
        comparison_with_greedy_degree_fill_in,
        "comparison_with_greedy_degree_fill_in",
    ),
    (
        comparison_of_spanning_tree_construction,
        "comparison_of_spanning_tree_construction",
    ),
];

pub fn comparison_of_edge_weights() -> Vec<HeuristicTypes> {
    vec![MSTreILeDif, MSTreINegIn, MSTreIUnion, MSTreIDisjU]
}

pub fn comparison_of_combined_edge_weights() -> Vec<HeuristicTypes> {
    vec![MSTreINegIn, MSTreINiTLd, MSTreILdTNi]
}

pub fn comparison_of_spanning_tree_construction() -> Vec<HeuristicTypes> {
    vec![MSTreINiTLd, FilWhINiTLd, FWhUEINiTLd, FWBagINonee]
}

pub fn comparison_with_greedy_degree_fill_in() -> Vec<HeuristicTypes> {
    vec![FilWhINiTLd, GreedyDegreeFillIn]
}

impl std::fmt::Display for HeuristicTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let display_string = match self {
            MSTreIUnion => "MTrUn".to_string(),
            MSTreIDisjU => "MTrDU".to_string(),
            MSTreINegIn => "MTrNi".to_string(),
            FilWhINegIn => "FiWhNi".to_string(),
            MSTreILeDif => "MTrLd".to_string(),
            FilWhILeDif => "FiWhLd".to_string(),
            MSTreINiTLd => "MTrNiTLd".to_string(),
            FilWhINiTLd => "FiWhNiTLd".to_string(),
            FWhUEINiTLd => "FWUNiTLd".to_string(),
            MSTreILdTNi => "MTrLdTNi".to_string(),
            FilWhILdTNi => "FiWhLdTNi".to_string(),
            FiWhTINiTLd => "FWTNiTLd".to_string(),
            FWBagINonee => "FWB".to_string(),
            MSTreINiTLdIBC(clique_bound) => format!("MTrNiTLdBC {}", clique_bound),
            FilWhINiTLdIBC(clique_bound) => format!("FiWhLdTNiBC {}", clique_bound),
            FiWhTINiTLdIBC(clique_bound) => format!("FWTNiTLd {}", clique_bound),
            GreedyDegreeFillIn => format!("GreedyDegreeFillIn"),
        };
        write!(f, "{}", display_string)
    }
}

pub enum EdgeWeightTypes<S> {
    ReturnI32(fn(&HashSet<NodeIndex, S>, &HashSet<NodeIndex, S>) -> i32),
    ReturnI32Tuple(fn(&HashSet<NodeIndex, S>, &HashSet<NodeIndex, S>) -> (i32, i32)),
}

use std::{collections::HashSet, error::Error, fs::File, hash::BuildHasher};

pub fn heuristic_to_spanning_tree_computation_type_and_edge_weight_heuristic<
    S: BuildHasher + Default,
>(
    heuristic: &HeuristicTypes,
) -> Option<(
    treewidth_heuristic_clique_graph::TreewidthComputationMethod,
    EdgeWeightTypes<S>,
)> {
    use treewidth_heuristic_clique_graph::TreewidthComputationMethod::*;
    use treewidth_heuristic_clique_graph::*;
    use EdgeWeightTypes::*;
    match heuristic {
        MSTreIUnion => Some((MSTAndUseTreeStructure, ReturnI32(union))),
        MSTreIDisjU => Some((MSTAndUseTreeStructure, ReturnI32(disjoint_union))),
        MSTreINegIn => Some((MSTAndUseTreeStructure, ReturnI32(negative_intersection))),
        FilWhINegIn => Some((FillWhilstMST, ReturnI32(negative_intersection))),
        MSTreILeDif => Some((MSTAndUseTreeStructure, ReturnI32(least_difference))),
        FilWhILeDif => Some((FillWhilstMST, ReturnI32(least_difference))),
        MSTreILdTNi => Some((
            MSTAndUseTreeStructure,
            EdgeWeightTypes::ReturnI32Tuple(least_difference_then_negative_intersection),
        )),
        FilWhILdTNi => Some((
            FillWhilstMST,
            EdgeWeightTypes::ReturnI32Tuple(least_difference_then_negative_intersection),
        )),
        MSTreINiTLd => Some((
            MSTAndUseTreeStructure,
            EdgeWeightTypes::ReturnI32Tuple(negative_intersection_then_least_difference),
        )),
        FilWhINiTLd => Some((
            FillWhilstMST,
            EdgeWeightTypes::ReturnI32Tuple(negative_intersection_then_least_difference),
        )),
        FWhUEINiTLd => Some((
            FillWhilstMSTEdgeUpdate,
            EdgeWeightTypes::ReturnI32Tuple(negative_intersection_then_least_difference),
        )),
        FiWhTINiTLd => Some((
            FillWhilstMSTTree,
            EdgeWeightTypes::ReturnI32Tuple(negative_intersection_then_least_difference),
        )),
        FWBagINonee => Some((
            FillWhilstMSTBagSize,
            EdgeWeightTypes::ReturnI32Tuple(negative_intersection_then_least_difference),
        )),
        MSTreINiTLdIBC(_) => Some((
            MSTAndUseTreeStructure,
            EdgeWeightTypes::ReturnI32Tuple(negative_intersection_then_least_difference),
        )),
        FilWhINiTLdIBC(_) => Some((
            FillWhilstMST,
            EdgeWeightTypes::ReturnI32Tuple(negative_intersection_then_least_difference),
        )),
        FiWhTINiTLdIBC(_) => Some((
            FillWhilstMSTTree,
            EdgeWeightTypes::ReturnI32Tuple(negative_intersection_then_least_difference),
        )),
        GreedyDegreeFillIn => None,
    }
}

// DEBUG
// pub fn heuristic_to_computation_type(
//     heuristic: &HeuristicTypes,
// ) -> treewidth_heuristic_clique_graph::TreewidthComputationMethod {
//     match heuristic {
//         MSTreIUnion => MSTAndUseTreeStructure,
//         MSTreIDisjU => MSTAndUseTreeStructure,
//         MSTreINegIn => MSTAndUseTreeStructure,
//         FilWhINegIn => FillWhilstMST,
//         MSTreILeDif => MSTAndUseTreeStructure,
//         FilWhILeDif => FillWhilstMST,
//         MSTreILdTNi => MSTAndUseTreeStructure,
//         FilWhILdTNi => FillWhilstMST,
//         MSTreINiTLd => MSTAndUseTreeStructure,
//         FilWhINiTLd => FillWhilstMST,
//         FWhUEINiTLd => FillWhilstMSTEdgeUpdate,
//         FiWhTINiTLd => FillWhilstMSTTree,
//         FWBagINonee => FillWhilstMSTBagSize,
//         MSTreINiTLdIBC(_) => MSTAndUseTreeStructure,
//         FilWhINiTLdIBC(_) => FillWhilstMST,
//         FiWhTINiTLdIBC(_) => FillWhilstMSTTree,
//         GreedyDegreeFillIn => None,
//     }
// }

pub fn heuristic_to_clique_bound(heuristic: &HeuristicTypes) -> Option<usize> {
    match heuristic {
        MSTreIUnion => None,
        MSTreIDisjU => None,
        MSTreINegIn => None,
        FilWhINegIn => None,
        MSTreILeDif => None,
        FilWhILeDif => None,
        MSTreILdTNi => None,
        FilWhILdTNi => None,
        MSTreINiTLd => None,
        FilWhINiTLd => None,
        FWhUEINiTLd => None,
        FiWhTINiTLd => None,
        FWBagINonee => None,
        MSTreINiTLdIBC(clique_bound) => Some(*clique_bound),
        FilWhINiTLdIBC(clique_bound) => Some(*clique_bound),
        FiWhTINiTLdIBC(clique_bound) => Some(*clique_bound),
        GreedyDegreeFillIn => None,
    }
}

/// Per Run Bound&Runtime Data Header should be of the form
/// Graph HeuristicVersion1 HeuristicVersion1 ... HeuristicVersion1 (number_of_runs_per_graph often) HeuristicVersion2 ...
///
/// Per Run Bound&Runtime Data should be of the form
/// Graph name HeuristicVersion1ComputedBoundFirstRun HeuristicVersion1ComputedBoundSecondRun ... HeuristicVersion1ComputedBoundNumber_of_runs_per_graph-thRun HeuristicVersion2 ....
pub fn write_to_csv(
    per_run_bound_data: &mut Vec<String>,
    per_run_runtime_data: &mut Vec<String>,
    average_bound_writer: &mut Writer<File>,
    per_run_bound_writer: &mut Writer<File>,
    average_runtime_writer: &mut Writer<File>,
    per_run_runtime_writer: &mut Writer<File>,
    number_of_runs_per_graph: usize,
    header: bool,
) -> Result<(), Box<dyn Error>> {
    if header {
        let mut average_bound_data: Vec<_> = Vec::new();
        let mut average_runtime_data: Vec<_> = Vec::new();
        let mut offset_counter = 1;

        println!(
            "Per Run Bound Data Header length: {:?}",
            per_run_bound_data.len()
        );

        for i in 0..per_run_bound_data.len() {
            if i == 0 || i == 1 {
                average_bound_data.push(
                    per_run_bound_data
                        .get(i)
                        .expect("Index should be in bound by loop invariant")
                        .to_owned(),
                );
                average_runtime_data.push(
                    per_run_runtime_data
                        .get(i)
                        .expect("Index should be in bound by loop invariant")
                        .to_owned(),
                );
            } else {
                if offset_counter == number_of_runs_per_graph {
                    average_bound_data.push(
                        per_run_bound_data
                            .get(i)
                            .expect("Index should be in bound by loop invariant")
                            .to_owned(),
                    );
                    average_runtime_data.push(
                        per_run_runtime_data
                            .get(i)
                            .expect("Index should be in bound by loop invariant")
                            .to_owned(),
                    );
                    offset_counter = 1;
                } else {
                    offset_counter += 1;
                }
            }
        }
        println!("Writing to writers");
        println!("Trying to write: {:?}", average_bound_data);
        per_run_bound_writer.write_record(per_run_bound_data)?;
        per_run_runtime_writer.write_record(per_run_runtime_data)?;

        average_bound_writer.write_record(average_bound_data)?;
        average_runtime_writer.write_record(average_runtime_data)?;
    } else {
        let mut average_bound_data: Vec<f64> = Vec::new();
        let mut average_bound_header: Vec<String> = Vec::new();
        let mut average_runtime_data: Vec<f64> = Vec::new();
        let mut average_runtime_header: Vec<String> = Vec::new();
        let mut offset_counter = 1;
        let mut average_runtime: f64 = 0.0;
        let mut average_bound: f64 = 0.0;

        for i in 0..per_run_bound_data.len() {
            if i == 0 || i == 1 {
                average_bound_header.push(
                    per_run_bound_data
                        .get(i)
                        .expect("Index should be in bound by loop invariant")
                        .to_owned(),
                );
                average_runtime_header.push(
                    per_run_runtime_data
                        .get(i)
                        .expect("Index should be in bound by loop invariant")
                        .to_owned(),
                );
            } else {
                if offset_counter == number_of_runs_per_graph {
                    average_runtime /= number_of_runs_per_graph as f64;
                    average_bound /= number_of_runs_per_graph as f64;

                    average_bound_data.push(average_bound);
                    average_runtime_data.push(average_runtime);
                    average_bound = per_run_bound_data
                        .get(i)
                        .expect("Index should be in bound by loop invariant")
                        .parse::<f64>()
                        .expect("Entries of data vectors should be valid f64");
                    average_runtime = per_run_runtime_data
                        .get(i)
                        .expect("Index should be in bound by loop invariant")
                        .parse::<f64>()
                        .expect("Entries of data vectors should be valid f64");

                    offset_counter = 1;
                } else {
                    average_bound += per_run_bound_data
                        .get(i)
                        .expect("Index should be in bound by loop invariant")
                        .parse::<f64>()
                        .expect("Entries of data vectors should be valid f64");
                    average_runtime += per_run_runtime_data
                        .get(i)
                        .expect("Index should be in bound by loop invariant")
                        .parse::<f64>()
                        .expect("Entries of data vectors should be valid f64");
                    offset_counter += 1;

                    // if i == per_run_bound_data.len() - 1 {
                    //     average_bound_data.push(average_bound);
                    //     average_runtime_data.push(average_runtime);
                    // }
                }
            }
        }

        let average_bound_data: Vec<String> = average_bound_header
            .into_iter()
            .chain(average_bound_data.iter().map(|u| u.to_string()))
            .collect();
        let average_runtime_data: Vec<_> = average_runtime_header
            .into_iter()
            .chain(average_runtime_data.iter().map(|u| u.to_string()))
            .collect();

        per_run_bound_writer.write_record(per_run_bound_data)?;
        per_run_runtime_writer.write_record(per_run_runtime_data)?;

        average_bound_writer.write_record(average_bound_data)?;
        average_runtime_writer.write_record(average_runtime_data)?;
    }
    per_run_bound_writer.flush()?;
    per_run_runtime_writer.flush()?;
    average_bound_writer.flush()?;
    average_runtime_writer.flush()?;

    Ok(())
}
