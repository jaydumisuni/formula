from pathlib import Path


def replace_exact(path: str, old: str, new: str) -> None:
    target = Path(path)
    text = target.read_text()
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{path}: expected one exact patch match, found {count}")
    target.write_text(text.replace(old, new))


replace_exact(
    "crates/formula-engine/src/affine_polynomial.rs",
    """            let pivot = matrix[pivot_row][column];
            for entry in column..=variables {
                matrix[pivot_row][entry] = matrix[pivot_row][entry].checked_div(pivot)?;
            }

            for row in 0..matrix.len() {
                if row == pivot_row {
                    continue;
                }
                let factor = matrix[row][column];
                if factor.is_zero() {
                    continue;
                }
                for entry in column..=variables {
                    let scaled = factor.checked_mul(matrix[pivot_row][entry])?;
                    matrix[row][entry] = matrix[row][entry].checked_sub(scaled)?;
                }
            }
""",
    """            let pivot = matrix[pivot_row][column];
            for entry in &mut matrix[pivot_row][column..=variables] {
                *entry = (*entry).checked_div(pivot)?;
            }

            let pivot_tail = matrix[pivot_row][column..=variables].to_vec();
            for row in 0..matrix.len() {
                if row == pivot_row {
                    continue;
                }
                let factor = matrix[row][column];
                if factor.is_zero() {
                    continue;
                }
                for (entry, pivot_entry) in matrix[row][column..=variables]
                    .iter_mut()
                    .zip(&pivot_tail)
                {
                    let scaled = factor.checked_mul(*pivot_entry)?;
                    *entry = (*entry).checked_sub(scaled)?;
                }
            }
""",
)

replace_exact(
    "crates/formula-engine/src/search_policy.rs",
    """impl<T: Clone> FairRoundRobin<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            next_index: 0,
        }
    }

    pub fn next(&mut self) -> Option<T> {
        if self.items.is_empty() {
            return None;
        }
        let item = self.items[self.next_index].clone();
        self.next_index = (self.next_index + 1) % self.items.len();
        Some(item)
    }
}
""",
    """impl<T> FairRoundRobin<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            next_index: 0,
        }
    }
}

impl<T: Clone> Iterator for FairRoundRobin<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() {
            return None;
        }
        let item = self.items[self.next_index].clone();
        self.next_index = (self.next_index + 1) % self.items.len();
        Some(item)
    }
}
""",
)
