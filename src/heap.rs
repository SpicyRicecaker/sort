use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

pub const PADDING: u32 = 1;

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

#[derive(Debug, Clone)]
pub struct State {
    pub pointer: usize,
    pub data: Vec<i32>,
}

#[derive(Debug)]
pub struct Heap {
    data_states: VecDeque<State>,
    max: Option<i32>,
    static_cooldown: Duration,
    time: Instant,
}

impl Heap {
    fn heap_sort_visual(&mut self, data: &mut [i32]) {
        // start at last node
        // this is the part that's log(n) complexity, it seems
        // dbg!(&data);
        for i in (0..=(data.len() - 1) / 2).rev() {
            self.heapify_visual(i, data, data.len());
        }
        // dbg!("after initial heapify", &data);
        for i in (0..data.len()).rev() {
            data.swap(0, i);
            self.heapify_visual(0, data, i);
        }
    }

    fn heapify_visual(&mut self, i: usize, data: &mut [i32], size: usize) {
        // every time heapify is called, add the current state to global state
        self.data_states.push_back(State {
            pointer: i,
            data: data.to_vec(),
        });
        // dbg!(&data);
        // check left and right children, but ensure that they exist
        let left_idx = i * 2;
        let right_idx = i * 2 + 1;

        //  1  -  0
        // 2 3 - 1 2
        // size = 3
        let mut largest_idx = i; // 0
        if left_idx < size && data[left_idx] > data[largest_idx] {
            // 1 < 3 && 2 > 1 -> yes
            largest_idx = left_idx; // 1
        }
        if right_idx < size && data[right_idx] > data[largest_idx] {
            // 2 < 3 && 3 > 2 -> yes
            largest_idx = right_idx; // 2
        }
        // 2 != 0
        if largest_idx != i {
            data.swap(largest_idx, i);
            //  3  -  0
            // 2 1 - 1  2
            self.heapify_visual(largest_idx, data, size);
        }
    }

    pub fn new(mut data: Vec<i32>) -> Self {
        let mut heap = Heap {
            data_states: VecDeque::new(),
            max: None,
            static_cooldown: Duration::from_millis(10),
            time: Instant::now(),
        };
        heap.heap_sort_visual(&mut data);
        heap.max = data.last().cloned();
        heap
    }

    pub fn tick(&mut self) {
        // if self.time.elapsed() > self.static_cooldown {
        // self.time = Instant::now();
        if self.data_states.len() > 1 {
            // dbg!("popping", self.time.elapsed());
            self.data_states.pop_front();
        }
        // }
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>, size_pillar_x: u32) {
        if let Some(last) = self.data_states.front() {
            for (i, element) in last.data.iter().enumerate() {
                if i == last.pointer {
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
                } else {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                }
                canvas
                    .fill_rect(Rect::new(
                        (i as u32 * size_pillar_x + PADDING) as i32,
                        0,
                        size_pillar_x - 2 * PADDING,
                        ((*element as f32 / self.max.unwrap_or(100) as f32) * 720_f32).floor()
                            as u32,
                    ))
                    .unwrap();
            }
        }
    }
}

// everytime heapify is called, additionally print the pointer
fn heapify(i: usize, data: &mut [i32], size: usize) {
    // dbg!(&data);
    // check left and right children, but ensure that they exist
    let left_idx = i * 2;
    let right_idx = i * 2 + 1;

    //  1  -  0
    // 2 3 - 1 2
    // size = 3
    let mut largest_idx = i; // 0
    if left_idx < size && data[left_idx] > data[largest_idx] {
        // 1 < 3 && 2 > 1 -> yes
        largest_idx = left_idx; // 1
    }
    if right_idx < size && data[right_idx] > data[largest_idx] {
        // 2 < 3 && 3 > 2 -> yes
        largest_idx = right_idx; // 2
    }
    // 2 != 0
    if largest_idx != i {
        data.swap(largest_idx, i);
        //  3  -  0
        // 2 1 - 1  2
        heapify(largest_idx, data, size);
    }
}

#[inline(always)]
pub fn heap_sort(data: &mut [i32]) {
    // start at last node
    // this is the part that's log(n) complexity, it seems
    // dbg!(&data);
    for i in (0..=(data.len() - 1) / 2).rev() {
        heapify(i, data, data.len());
    }
    dbg!("heap after heapify", &data);
    // dbg!("after initial heapify", &data);
    for i in (0..data.len()).rev() {
        data.swap(0, i);
        heapify(0, data, i);
    }
}
