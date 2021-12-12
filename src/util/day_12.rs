use std::collections::HashMap;
use super::Part;

pub fn solve(input : String, part: Part) -> String {

    let lines:Vec<&str> = input.lines()
        .collect();

    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

#[derive(Debug)]
struct Graph {
    edges:HashMap<String,Vec<String>>
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Edge {
    from:String,
    to:String,
}


impl Graph {
    fn new(edges:Vec<Edge>) -> Graph {
        let mut graph = Graph{edges:HashMap::new()};
        edges.iter().for_each(|edge| {
            // Insert from -> to
            if !graph.edges.contains_key(edge.from.as_str())  {
                graph.edges.insert(edge.from.clone(), vec![edge.to.clone()]);
            } else {
                graph.edges.get_mut(edge.from.as_str()).unwrap().push(edge.to.to_string());
            }

            let reverse_edge = edge.revert();
            if !reverse_edge.to.as_str().eq("start") {
                if !graph.edges.contains_key(reverse_edge.from.as_str())  {
                    graph.edges.insert(reverse_edge.from.clone(), vec![reverse_edge.to.to_string()]);
                } else {
                    graph.edges.get_mut(reverse_edge.from.as_str()).unwrap().push(reverse_edge.to.to_string());
                }
            }


        });
        graph
    }

    fn next(&self,current_node:&str, visited:&HashMap<&str, u32>, part1:bool) -> Vec<&str> {
        if part1 {
            self.next_part1(current_node, visited)
        } else {
            self.next_part2(current_node, visited)
        }
    }

    fn next_part1(&self,current_node:&str, visited:&HashMap<&str,u32>) -> Vec<&str> {
        let edges = self.edges.get(&current_node.to_string()).unwrap();
        let candidates : Vec<&str> = edges.iter()
            .filter(|node| node.as_str().ne("start") && !visited.contains_key(&node.as_str()))
            .map(|node| node.as_str())
            .collect();
        candidates
    }

    fn next_part2(&self,current_node:&str, visited:&HashMap<&str,u32>) -> Vec<&str> {
        let edges = self.edges.get(&current_node.to_string()).unwrap();
        let used_double_visit = visited.contains_key(current_node) || visited.iter().any(|(_, cnt)| *cnt > 1);

        if used_double_visit {
            // Used double visit
            self.next_part1(current_node, visited)
        } else {
            // Allow all
            edges.iter()
                .filter(|node | node.as_str().ne("start"))
                .map(|node| node.as_str())
                .collect()
        }
    }


}

impl Edge {
    fn new(line:&str) -> Edge {
        let mut it = line.split('-');
        let from = it.next().unwrap();
        let to = it.next().unwrap();
        Edge{from:from.to_string(),to:to.to_string()}
    }

    fn revert(&self) -> Edge {
        Edge{from:self.to.clone(),to:self.from.clone()}
    }
}



fn find_paths(graph:&Graph, current_node:&str, visited:HashMap<&str,u32>, path:Vec<String>, solutions:&mut Vec<Vec<String>>, part1:bool) {
    let next_steps = graph.next(current_node, &visited, part1);

    // Add current node to visited if its a small cave
    let small_cave = current_node.chars().all(|ch| ch.is_lowercase());
    let mut next_path = path.clone();

    next_path.push(current_node.to_string());

    if current_node.eq("end") {
        // Add solution
        //println!("====> {:?}", next_path);
        solutions.push(next_path);

    } else {
        for next_step in next_steps {
            let mut next_visited = visited.clone();

            if small_cave {
                if next_visited.contains_key(current_node) {
                    let cnt = next_visited.get_mut(current_node).unwrap();
                    *cnt += 1;
                } else {
                    next_visited.insert(current_node, 1);
                }
            }

            // Next step
            find_paths(graph, next_step, next_visited.clone(), next_path.clone(), solutions, part1);
        }
    }
}

fn part1(lines:Vec<&str>) -> String {
    let edges:Vec<Edge> = lines.iter().map(|line| Edge::new(line)).collect();
    let graph = Graph::new(edges);

    let mut solutions = vec![];
    find_paths(&graph, "start", HashMap::new(), vec![], &mut solutions, true);

    solutions.len().to_string()
}


fn part2(lines:Vec<&str>) -> String {
    let edges:Vec<Edge> = lines.iter().map(|line| Edge::new(line)).collect();
    let graph = Graph::new(edges);

    let mut solutions = vec![];
    find_paths(&graph, "start", HashMap::new(), vec![], &mut solutions, false);

    solutions.len().to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        assert_eq!("10", solve(input.to_string(), Part1));
    }

    #[test]
    fn test11() {

        let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

        assert_eq!("19", solve(input.to_string(), Part1));
    }

    #[test]
    fn test12() {

        let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

        assert_eq!("226", solve(input.to_string(), Part1));
    }


    #[test]
    fn test_part1() {
        let input = include_str!("../../input_12.txt");

        assert_eq!("4691", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!("36", solve(input.to_string(), Part2));
    }

    #[test]
    fn test21() {

        let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

        assert_eq!("103", solve(input.to_string(), Part2));
    }

    #[test]
    fn test22() {

        let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

        assert_eq!("3509", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_12.txt");

        assert_eq!("140718", solve(input.to_string(), Part2));
    }

}
