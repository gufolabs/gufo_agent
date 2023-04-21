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
//         name: "requests_total",
//         help: "Total DNS requests performed",
//         value: Value::Counter(v),
//         labels: Labels::empty()
//     }
// }
// ```
// Example:
// `counter!(my_counter, "Help string", query);`
// expands to
// ```
// fn my_counter<K1: ToString>(v: u64, l_query: K1) -> Measure {
//     Measure {
//         name: "requests_total",
//         help: "Total DNS requests performed",
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
                name: stringify!($name),
                help: $help,
                value: common::Value::Counter(v),
                labels: common::Labels::empty(),
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
                    name: stringify!($name),
                    help: $help,
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

// Define gauge
// Example:
// `gauge!(my_gauge, "Help string");`
// expands to
// ```
// fn my_gauge(v: u64) -> Measure {
//     Measure {
//         name: "requests_total",
//         help: "Total DNS requests performed",
//         value: Value::Gauge(v),
//         labels: Labels::empty()
//     }
// }
// ```
// Example:
// `gauge!(my_gauge, "Help string", query);`
// expands to
// ```
// fn my_gauge<K1: ToString>(v: u64, l_query: K1) -> Measure {
//     Measure {
//         name: "requests_total",
//         help: "Total DNS requests performed",
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
                name: stringify!($name),
                help: $help,
                value: common::Value::Gauge(v),
                labels: common::Labels::empty(),
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
                    name: stringify!($name),
                    help: $help,
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

// Define siged gauge
// Example:
// `gauge_i!(my_gauge, "Help string");`
// expands to
// ```
// fn my_gauge(v: i64) -> Measure {
//     Measure {
//         name: "requests_total",
//         help: "Total DNS requests performed",
//         value: Value::GaugeI(v),
//         labels: Labels::empty()
//     }
// }
// ```
// Example:
// `gauge_i!(my_gauge, "Help string", query);`
// expands to
// ```
// fn my_gauge<K1: ToString>(v: i64, l_query: K1) -> Measure {
//     Measure {
//         name: "requests_total",
//         help: "Total DNS requests performed",
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
                name: stringify!($name),
                help: $help,
                value: common::Value::GaugeI(v),
                labels: common::Labels::empty(),
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
                    name: stringify!($name),
                    help: $help,
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
