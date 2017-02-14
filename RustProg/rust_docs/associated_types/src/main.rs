struct Node;
struct Edge;
struct Mygraph;

trait Graph{
	type N;
	type E;
	fn has_edge(&self,&Self::N,&Self::E) -> bool;
	fn edges(&self,&Self::N) -> Vec<Self::E>;
}

impl Graph for Mygraph{
	type N = Node;
	type E = Edge;

	fn has_edge(&self,n1:&Node,n2:&Edge) -> bool{
		true
	}
	fn edges(&self,n:&Node) -> Vec<Edge> {
		Vec::new()
	}
}

fn main() {
    let graph = Mygraph;
    let obj = Box::new(graph) as Box<Graph<N=Node,E=Edge>>;
}

