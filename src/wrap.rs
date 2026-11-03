//! Line breaking shared by every label that wraps (report M7): node
//! labels ([`crate::layout::wrap_label`]), markdown strings
//! ([`crate::markdown`]) and the fixed-column text of journeys and
//! timelines ([`crate::layout`]'s `wrap_text`). Each caller splits its
//! text into words its own way (markdown words carry styles) and says
//! how wide a run of words is on one line; the breakers here only pick
//! where the lines start. Words are never split: a word wider than the
//! limit keeps a line to itself, as mermaid's wrapped labels do.

/// Most words [`balanced`] balances: past it a label is a paragraph,
/// not a label, and greedy filling (linear) is good enough.
const BALANCE_MAX_WORDS: usize = 64;

/// Where the lines of `n` words start, filling each line greedily while
/// the next word fits in `width` (`line_w(i, j)`: how wide words `i..j`
/// are on one line). The first entry is always 0; empty for no words.
pub(crate) fn greedy(n: usize, width: f64, line_w: impl Fn(usize, usize) -> f64) -> Vec<usize> {
    let mut starts = Vec::new();
    if n == 0 {
        return starts;
    }
    starts.push(0);
    let mut from = 0;
    for j in 1..n {
        if line_w(from, j + 1) > width {
            starts.push(j);
            from = j;
        }
    }
    starts
}

/// Where the lines of `n` words start when they wrap at `width`
/// balanced: as few lines as [`greedy`] filling needs, and of all the
/// ways to break into that many lines (none wider than `width` but for
/// a lone over-long word) the one whose widest line is narrowest, the
/// others as even as that allows, ties to the earliest breaks. So a label never ends on a stranded short
/// word when moving words down evens the lines out ("Return documents
/// to | the applicant", not "Return documents to the | applicant").
/// Over [`BALANCE_MAX_WORDS`] words, greedy.
pub(crate) fn balanced(n: usize, width: f64, line_w: impl Fn(usize, usize) -> f64) -> Vec<usize> {
    let starts = greedy(n, width, &line_w);
    let k = starts.len();
    if k <= 1 || n > BALANCE_MAX_WORDS {
        return starts;
    }
    into_lines(n, k, width, &line_w).unwrap_or(starts)
}

/// Where the lines of `n` words start when broken into exactly `k`
/// lines, the widest as narrow as can be and the rest as even as that
/// allows (ties to the earliest breaks),
/// no line wider than `width` but for a lone word (pass infinity for no
/// limit); `None` when no such break exists (`k` above `n`, or too few
/// lines for `width`), or past [`BALANCE_MAX_WORDS`] words.
pub(crate) fn into_lines(
    n: usize,
    k: usize,
    width: f64,
    line_w: impl Fn(usize, usize) -> f64,
) -> Option<Vec<usize>> {
    if k == 0 || k > n || n > BALANCE_MAX_WORDS {
        return None;
    }
    // Width of words i..j on one line, or infinite when it overflows
    // (a lone word may).
    let mut w = vec![vec![f64::INFINITY; n + 1]; n];
    for (i, row) in w.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate().skip(i + 1) {
            let lw = line_w(i, j);
            if lw > width && j > i + 1 {
                break;
            }
            *cell = lw;
        }
    }
    // best[m][j]: narrowest widest line over words 0..j in m lines, and
    // where its last line starts.
    let mut best = vec![vec![(f64::INFINITY, 0usize); n + 1]; k + 1];
    best[0][0] = (0.0, 0);
    for m in 1..=k {
        for j in m..=n {
            for i in (m - 1)..j {
                let prev = best[m - 1][i].0;
                if !prev.is_finite() || !w[i][j].is_finite() {
                    continue;
                }
                let cand = prev.max(w[i][j]);
                if cand < best[m][j].0 {
                    best[m][j] = (cand, i);
                }
            }
        }
    }
    let widest = best[k][n].0;
    if !widest.is_finite() {
        return None;
    }
    // Of the breaks whose widest line is that narrow, the most even:
    // least summed squared shortfall of each line from the widest
    // ("Are all documents | complete and | compliant with | the
    // rules?", not "… | complete | and compliant | …").
    let cap = widest + 1e-9;
    let mut even = vec![vec![(f64::INFINITY, 0usize); n + 1]; k + 1];
    even[0][0] = (0.0, 0);
    for m in 1..=k {
        for j in m..=n {
            for i in (m - 1)..j {
                let prev = even[m - 1][i].0;
                if !prev.is_finite() || w[i][j] > cap {
                    continue;
                }
                let cand = prev + (widest - w[i][j]).powi(2);
                if cand < even[m][j].0 {
                    even[m][j] = (cand, i);
                }
            }
        }
    }
    let mut out = vec![0; k];
    let mut j = n;
    for m in (1..=k).rev() {
        let i = even[m][j].1;
        out[m - 1] = i;
        j = i;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Words of the given widths, one px per space between them.
    fn widths(ws: &[f64]) -> impl Fn(usize, usize) -> f64 + '_ {
        move |i, j| ws[i..j].iter().sum::<f64>() + (j - i - 1) as f64
    }

    #[test]
    fn greedy_fills_lines_and_keeps_long_words_whole() {
        assert_eq!(greedy(0, 10.0, |_, _| 0.0), Vec::<usize>::new());
        assert_eq!(greedy(3, 100.0, widths(&[10.0, 10.0, 10.0])), vec![0]);
        assert_eq!(greedy(3, 21.0, widths(&[10.0, 10.0, 10.0])), vec![0, 2]);
        // An over-long word sits alone, unsplit.
        assert_eq!(greedy(3, 15.0, widths(&[10.0, 50.0, 10.0])), vec![0, 1, 2]);
    }

    #[test]
    fn balanced_keeps_the_line_count_and_evens_the_lines() {
        // Greedy strands the last word: [30 30 30] [10]; balanced moves
        // a word down: [30 30] [30 10].
        let ws = [30.0, 30.0, 30.0, 10.0];
        assert_eq!(greedy(4, 95.0, widths(&ws)), vec![0, 3]);
        assert_eq!(balanced(4, 95.0, widths(&ws)), vec![0, 2]);
        // One line when it fits; a lone over-long word stays alone.
        assert_eq!(balanced(4, 200.0, widths(&ws)), vec![0]);
        assert_eq!(
            balanced(3, 20.0, widths(&[10.0, 50.0, 10.0])),
            vec![0, 1, 2]
        );
        // Exactly k lines, the widest as narrow as can be.
        assert_eq!(
            into_lines(4, 3, f64::INFINITY, widths(&ws)),
            Some(vec![0, 1, 2])
        );
        assert_eq!(into_lines(4, 5, f64::INFINITY, widths(&ws)), None);
        assert_eq!(into_lines(4, 1, 50.0, widths(&ws)), None);
        // Never more lines than greedy, never a line over the width but
        // a lone word.
        let ws = [12.0, 40.0, 7.0, 33.0, 21.0, 5.0, 60.0, 9.0];
        for width in [40.0, 55.0, 70.0, 90.0, 130.0] {
            let g = greedy(ws.len(), width, widths(&ws));
            let b = balanced(ws.len(), width, widths(&ws));
            assert_eq!(b.len(), g.len(), "{width}");
            let ends: Vec<usize> = b.iter().skip(1).copied().chain([ws.len()]).collect();
            for (&s, &e) in b.iter().zip(&ends) {
                assert!(e > s);
                assert!(
                    e - s == 1 || widths(&ws)(s, e) <= width,
                    "{width}: {s}..{e}"
                );
            }
        }
    }
}
