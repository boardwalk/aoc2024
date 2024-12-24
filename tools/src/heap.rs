fn parent(idx: usize) -> Option<usize> {
    if idx > 0 {
        Some((idx - 1) / 2)
    } else {
        None
    }
}

fn left_child(idx: usize) -> usize {
    2 * idx + 1
}

fn right_child(idx: usize) -> usize {
    2 * idx + 2
}

// move an element in the tree up as needed
// this is a max heap (biggest element at root)
pub fn sift_up(heap: &mut [impl Ord], mut cur_idx: usize) -> usize {
    loop {
        let Some(parent_idx) = parent(cur_idx) else {
            return cur_idx;
        };

        if heap[cur_idx] >= heap[parent_idx] {
            break cur_idx;
        }

        heap.swap(cur_idx, parent_idx);
        cur_idx = parent_idx;
    }
}

// move an element in the tree down as needed
// this is a max heap (biggest element at root)
pub fn sift_down(heap: &mut [impl Ord], cur_idx: usize) {
    let left_child_idx = left_child(cur_idx);
    let right_child_idx = right_child(cur_idx);

    let mut largest_idx = cur_idx;

    if left_child_idx < heap.len() && heap[left_child_idx] < heap[largest_idx] {
        largest_idx = left_child_idx;
    }

    if right_child_idx < heap.len() && heap[right_child_idx] < heap[largest_idx] {
        largest_idx = right_child_idx;
    }

    if largest_idx != cur_idx {
        heap.swap(cur_idx, largest_idx);
        sift_down(heap, largest_idx);
    }
}

fn heap_validate(heap: &[impl Ord], cur_idx: usize) -> bool {
    // check left subtree
    let left_idx = left_child(cur_idx);
    let right_idx = left_child(cur_idx);

    if left_idx < heap.len() {
        if heap[cur_idx] > heap[left_idx] || !heap_validate(heap, left_idx) {
            return false;
        }
    }

    // check right subtree
    if right_idx < heap.len() {
        if heap[cur_idx] > heap[right_idx] || !heap_validate(heap, right_idx) {
            return false;
        }
    }

    true
}

pub fn heap_push<T: Ord>(heap: &mut Vec<T>, elem: T) -> usize {
    let mut cur_idx = heap.len();
    heap.push(elem);

    cur_idx = sift_up(heap, cur_idx);
    assert!(heap_validate(heap, 0));
    cur_idx
}

pub fn heap_pop<T: Ord>(heap: &mut Vec<T>) -> Option<T> {
    if heap.is_empty() {
        return None;
    }

    let elem = heap.swap_remove(0);
    sift_down(heap, 0);
    assert!(heap_validate(heap, 0));
    assert!(heap.iter().all(|e| *e >= elem));
    Some(elem)
}

pub fn heap_decrease<T: Ord>(heap: &mut [T], cur_idx: usize) -> usize {
    let cur_idx = sift_up(heap, cur_idx);
    assert!(heap_validate(heap, 0));

    cur_idx
}
