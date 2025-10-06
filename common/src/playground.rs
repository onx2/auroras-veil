// #[spacetimedb(reducer)]
// pub fn move_to(entity_id: u64, target_x: f32, target_z: f32) {
//     let start_chunk = encode(entity.x, entity.z);
//     let goal_chunk = encode(target_x, target_z);

//     let nav_nodes: Vec<NavNode> = NavNode::all()
//         .filter(|n| is_near_chunk(n.chunk_id, start_chunk, goal_chunk))
//         .collect();

//     // Build a lookup map for neighbors
//     let get_neighbors = |node: &NavNode| {
//         node.edges
//             .iter()
//             .filter_map(|edge| {
//                 nav_nodes
//                     .iter()
//                     .find(|n| n.chunk_id == edge.target_chunk && n.id == edge.target_node)
//                     .map(|n| ((n.chunk_id, n.id), edge.cost as u32))
//             })
//             .collect::<Vec<_>>()
//     };

//     if let Some((path, _cost)) = astar(
//         &(
//             start_chunk,
//             nearest_node_in_chunk(&nav_nodes, entity.x, entity.z),
//         ),
//         get_neighbors,
//         |n| heuristic(n, target_x, target_z),
//         |n| reached_goal(n, target_x, target_z),
//     ) {
//         MovementPath::insert(MovementPath {
//             entity_id,
//             steps: path
//                 .iter()
//                 .map(|(chunk_id, node_id)| {
//                     let node = nav_nodes
//                         .iter()
//                         .find(|n| n.chunk_id == *chunk_id && n.id == *node_id)
//                         .unwrap();
//                     (node.x, node.z)
//                 })
//                 .collect(),
//             current_step: 0,
//         });
//     }
// }
