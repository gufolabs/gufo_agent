// --------------------------------------------------------------------
// Gufo Agent: metrics helper macros
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

pub use paste::paste;

// Define counter
// Example:
// `counter!(my_counter, "Help string");`
// expands to
// ```
// fn my_counter(v: u64) -> Measure {
//     Measure {
//         name: "my_counter".to_string(),
//         help: "Help string".to_string(),
//         value: Value::Counter(v),
//         labels: Labels::default()
//     }
// }
// ```
// Example:
// `counter!(my_counter, "Help string", query);`
// expands to
// ```
// fn my_counter<K1: ToString>(v: u64, l_query: K1) -> Measure {
//     Measure {
//         name: "my_counter".to_string(),
//         help: "Help string".to_string(),
//         value: Value::Counter(v),
//         labels: vec![Labels::new("query", l_query)]
//     }
// }
// ```
#[macro_export]
macro_rules! counter {
    // Without labels
    ($name:ident, $help:literal) => {
        fn $name(v: u64) -> Measure {
            Measure {
                name: stringify!($name).to_string(),
                help: $help.to_string(),
                value: common::Value::Counter(v),
                labels: common::Labels::default(),
            }
        }
    };
    // With labels
    ($name:ident, $help:literal, $($label:ident),+) => {
        common::metrics::paste! {
            fn $name<$([< T $label >]: ToString),+>(
                v: u64, $([< l_ $label>]: [< T $label >]),+
            ) -> Measure
            where $([< T $label >]: Clone),+
            {
                Measure {
                    name: stringify!($name).to_string(),
                    help: $help.to_string(),
                    value: common::Value::Counter(v),
                    labels: common::Labels::new(
                        vec![
                            $(common::Label::new(stringify!($label), [< l_ $label >].clone()),)+
                        ]
                    ),
                }
            }
        }
    };
}

// Define float counter
// Example:
// `counter_f!(my_counter, "Help string");`
// expands to
// ```
// fn my_counter(v: f32) -> Measure {
//     Measure {
//         name: "my_counter".to_string(),
//         help: "Help string".to_string(),
//         value: Value::CounterF(v),
//         labels: Labels::default()
//     }
// }
// ```
// Example:
// `counter!(my_counter, "Help string", query);`
// expands to
// ```
// fn my_counter<K1: ToString>(v: f32, l_query: K1) -> Measure {
//     Measure {
//         name: "my_counter".to_string(),
//         help: "Help string".to_string(),
//         value: Value::CounterF(v),
//         labels: vec![Labels::new("query", l_query)]
//     }
// }
// ```
#[macro_export]
macro_rules! counter_f {
    // Without labels
    ($name:ident, $help:literal) => {
        fn $name(v: f32) -> Measure {
            Measure {
                name: stringify!($name).to_string(),
                help: $help.to_string(),
                value: common::Value::CounterF(v),
                labels: common::Labels::default(),
            }
        }
    };
    // With labels
    ($name:ident, $help:literal, $($label:ident),+) => {
        common::metrics::paste! {
            fn $name<$([< T $label >]: ToString),+>(
                v: f32, $([< l_ $label>]: [< T $label >]),+
            ) -> Measure
            where $([< T $label >]: Clone),+
            {
                Measure {
                    name: stringify!($name).to_string(),
                    help: $help.to_string(),
                    value: common::Value::CounterF(v),
                    labels: common::Labels::new(
                        vec![
                            $(common::Label::new(stringify!($label), [< l_ $label >].clone()),)+
                        ]
                    ),
                }
            }
        }
    };
}

// Define gauge
// Example:
// `gauge!(my_gauge, "Help string");`
// expands to
// ```
// fn my_gauge(v: u64) -> Measure {
//     Measure {
//         name: "my_gauge".to_string(),
//         help: "Help string".to_string(),
//         value: Value::Gauge(v),
//         labels: Labels::default()
//     }
// }
// ```
// Example:
// `gauge!(my_gauge, "Help string", query);`
// expands to
// ```
// fn my_gauge<K1: ToString>(v: u64, l_query: K1) -> Measure {
//     Measure {
//         name: "my_gauge".to_string(),
//         help: "Help string".to_string(),
//         value: Value::Gauge(v),
//         labels: vec![Labels::new("query", l_query)]
//     }
// }
// ```
#[macro_export]
macro_rules! gauge {
    // Without labels
    ($name:ident, $help:expr) => {
        fn $name(v: u64) -> Measure {
            Measure {
                name: stringify!($name).to_string(),
                help: $help.to_string(),
                value: common::Value::Gauge(v),
                labels: common::Labels::default(),
            }
        }
    };
    // With labels
    ($name:ident, $help:literal, $($label:ident),+) => {
        common::metrics::paste! {
            fn $name<$([< T $label >]: ToString),+>(
                v: u64, $([< l_ $label>]: [< T $label >]),+
            ) -> Measure
            where $([< T $label >]: Clone),+
            {
                Measure {
                    name: stringify!($name).to_string(),
                    help: $help.to_string(),
                    value: common::Value::Gauge(v),
                    labels: common::Labels::new(
                        vec![
                            $(common::Label::new(stringify!($label), [< l_ $label >].clone()),)+
                        ]
                    ),
                }
            }
        }
    };
}

// Define signed gauge
// Example:
// `gauge_i!(my_gauge, "Help string");`
// expands to
// ```
// fn my_gauge(v: i64) -> Measure {
//     Measure {
//         name: "my_gauge".to_string(),
//         help: "Help string".to_string(),
//         value: Value::GaugeI(v),
//         labels: Labels::default()
//     }
// }
// ```
// Example:
// `gauge_i!(my_gauge, "Help string", query);`
// expands to
// ```
// fn my_gauge<K1: ToString>(v: i64, l_query: K1) -> Measure {
//     Measure {
//         name: "my_gauge".to_string(),
//         help: "Help string".to_string(),
//         value: Value::GaugeI(v),
//         labels: vec![Labels::new("query", l_query)]
//     }
// }
// ```
#[macro_export]
macro_rules! gauge_i {
    // Without labels
    ($name:ident, $help:expr) => {
        fn $name(v: i64) -> Measure {
            Measure {
                name: stringify!($name).to_string(),
                help: $help.to_string(),
                value: common::Value::GaugeI(v),
                labels: common::Labels::default(),
            }
        }
    };
    // With labels
    ($name:ident, $help:literal, $($label:ident),+) => {
        common::metrics::paste! {
            fn $name<$([< T $label >]: ToString),+>(
                v: i64, $([< l_ $label>]: [< T $label >]),+
            ) -> Measure
            where $([< T $label >]: Clone),+
            {
                Measure {
                    name: stringify!($name).to_string(),
                    help: $help.to_string(),
                    value: common::Value::GaugeI(v),
                    labels: common::Labels::new(
                        vec![
                            $(common::Label::new(stringify!($label), [< l_ $label >].clone()),)+
                        ]
                    ),
                }
            }
        }
    };
}
// Define float gauge
// Example:
// `gauge_f!(my_gauge, "Help string");`
// expands to
// ```
// fn my_gauge(v: f32) -> Measure {
//     Measure {
//         name: "requests_total".to_string(),
//         help: "Total DNS requests performed".to_string(),
//         value: Value::GaugeF(v),
//         labels: Labels::default()
//     }
// }
// ```
// Example:
// `gauge_f!(my_gauge, "Help string", query);`
// expands to
// ```
// fn my_gauge<K1: ToString>(v: f32, l_query: K1) -> Measure {
//     Measure {
//         name: "requests_total".to_string(),
//         help: "Total DNS requests performed".to_string(),
//         value: Value::GaugeF(v),
//         labels: vec![Labels::new("query", l_query)]
//     }
// }
// ```
#[macro_export]
macro_rules! gauge_f {
    // Without labels
    ($name:ident, $help:expr) => {
        fn $name(v: f32) -> Measure {
            Measure {
                name: stringify!($name).to_string(),
                help: $help.to_string(),
                value: common::Value::GaugeF(v),
                labels: common::Labels::default(),
            }
        }
    };
    // With labels
    ($name:ident, $help:literal, $($label:ident),+) => {
        common::metrics::paste! {
            fn $name<$([< T $label >]: ToString),+>(
                v: f32, $([< l_ $label>]: [< T $label >]),+
            ) -> Measure
            where $([< T $label >]: Clone),+
            {
                Measure {
                    name: stringify!($name).to_string(),
                    help: $help.to_string(),
                    value: common::Value::GaugeF(v),
                    labels: common::Labels::new(
                        vec![
                            $(common::Label::new(stringify!($label), [< l_ $label >].clone()),)+
                        ]
                    ),
                }
            }
        }
    };
}
