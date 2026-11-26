//! A few back edges must not stretch the layering of a large flowchart.
//!
//! A layered graph (`LAYERS` x `WIDTH`, two forward edges per node) plus
//! a handful of edges from late layers back to early ones has the same
//! natural depth as without them: the back edges are the feedback arc
//! set, and reversing them restores the layering. A cycle breaker that
//! reverses forward edges instead made 50 back edges turn 50 layers
//! into 350 and the layout from 0.1 s into seconds.

use flowmaid::model::Document;
use flowmaid::parser::parse_document;

const LAYERS: usize = 50;
const WIDTH: usize = 20;

/// Deterministic LCG, so the graph is the same on every run.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self, n: usize) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 33) as usize) % n
    }
}

fn source(back_edges: usize) -> String {
    let mut rng = Lcg(1);
    let mut s = String::from("flowchart TD\n");
    for l in 0..LAYERS - 1 {
        for w in 0..WIDTH {
            for _ in 0..2 {
                s += &format!("N{l}_{w} --> N{}_{}\n", l + 1, rng.next(WIDTH));
            }
        }
    }
    for _ in 0..back_edges {
        let from = LAYERS / 2 + rng.next(LAYERS / 2);
        let to = rng.next(LAYERS / 2);
        s += &format!(
            "N{from}_{} --> N{to}_{}\n",
            rng.next(WIDTH),
            rng.next(WIDTH)
        );
    }
    s
}

fn layers(src: &str) -> usize {
    let Document::Flowchart(g) = parse_document(src).unwrap() else {
        panic!("not a flowchart")
    };
    let lo = flowmaid::layout::layout(&g);
    lo.nodes.iter().map(|p| p.layer).max().unwrap() + 1
}

#[test]
fn back_edges_do_not_stretch_the_layering() {
    let plain = layers(&source(0));
    let cyclic = layers(&source(50));
    assert!(
        cyclic <= plain * 3 / 2,
        "50 back edges took {plain} layers to {cyclic}"
    );
}
