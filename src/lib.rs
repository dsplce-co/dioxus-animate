use std::{rc::Rc, time::Duration};

use dioxus::{prelude::*, web::WebEventExt};

#[derive(Clone, PartialEq)]
pub enum Operation {
    Add(String),
    Remove(String),
    Group(Vec<Operation>),
}

fn operate(patient: &web_sys::Element, operation: &Operation) {
    match operation {
        Operation::Add(class) => {
            let classes = class.split_whitespace().collect::<Vec<&str>>();

            for class in classes {
                let _ = patient.class_list().add_1(class);
            }
        }
        Operation::Remove(class) => {
            let classes = class.split_whitespace().collect::<Vec<&str>>();

            for class in classes {
                let _ = patient.class_list().remove_1(class);
            }
        },
        Operation::Group(operations) => {
            for operation in operations {
                operate(patient, operation);
            }
        }
    }
}

#[macro_export]
macro_rules! use_animate {
    ( $($input:tt)* ) => {{
        let operations = use_signal(|| {
            let mut ops = Vec::new();
            $crate::__parse_animations!(ops, $($input)*);
            ops
        });

        $crate::_use_animate(operations.into())
    }};
}

#[macro_export]
macro_rules! __parse_animations {
    ($ops:ident, $t:expr => add($s:literal), $($rest:tt)*) => {
        $ops.push(($t, $crate::Operation::Add($s.to_string())));
        $crate::__parse_animations!($ops, $($rest)*);
    };

    ($ops:ident, $t:expr => remove($s:literal), $($rest:tt)*) => {
        $ops.push(($t, $crate::Operation::Remove($s.to_string())));
        $crate::__parse_animations!($ops, $($rest)*);
    };

    ($ops:ident, $t:expr => ($($inner:tt)*), $($rest:tt)*) => {
        $ops.push(($t, {
            let mut group_ops = Vec::new();
            $crate::__expand_inner!(group_ops, $($inner)*);
            $crate::Operation::Group(group_ops)
        }));
        $crate::__parse_animations!($ops, $($rest)*);
    };

    ($ops:ident, $t:expr => add($s:literal)) => {
        $ops.push(($t, $crate::Operation::Add($s.to_string())));
    };

    ($ops:ident, $t:expr => remove($s:literal)) => {
        $ops.push(($t, $crate::Operation::Remove($s.to_string())));
    };

    ($ops:ident, $t:expr => ($($inner:tt)*)) => {
        $ops.push(($t, {
            let mut group_ops = Vec::new();
            $crate::__expand_inner!(group_ops, $($inner)*);
            $crate::Operation::Group(group_ops)
        }));
    };

    ($ops:ident,) => {};
}

#[macro_export]
macro_rules! __expand_inner {
    ( $ops:ident, add($s:literal); $($rest:tt)* ) => {
        $ops.push($crate::Operation::Add($s.to_string()));
        $crate::__expand_inner!($ops, $($rest)*);
    };
    ( $ops:ident, remove($s:literal); $($rest:tt)* ) => {
        $ops.push($crate::Operation::Remove($s.to_string()));
        $crate::__expand_inner!($ops, $($rest)*);
    };
    ( $ops:ident, add($s:literal) ) => {
        $ops.push($crate::Operation::Add($s.to_string()));
    };
    ( $ops:ident, remove($s:literal) ) => {
        $ops.push($crate::Operation::Remove($s.to_string()));
    };
    ( $ops:ident, ) => {};
}

pub struct UseAnimate {
    ops: Vec<(u64, Operation)>
}

impl UseAnimate {
    pub fn start(&self, patient: ReadOnlySignal<Option<Rc<MountedData>>>) {
        let ops = self.ops.clone();

        spawn(async move {
            let Some(element) = patient() else {
                return;
            };

            let mut time_elapsed = 0;

            for (duration, operation) in ops.into_iter() {
                let interval = duration - time_elapsed;

                dioxus_time::sleep(Duration::from_millis(interval)).await;
                time_elapsed += interval;

                let web_element = element.as_web_event();
                operate(&web_element, &operation);
            }
        });
    }
}

pub fn _use_animate(ops: ReadOnlySignal<Vec<(u64, Operation)>>) -> UseAnimate {
    UseAnimate {
        ops: ops()
    }
}

pub mod prelude {
    #[macro_use]
    pub use crate::use_animate;
}
