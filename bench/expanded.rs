#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod fork {
    #[cfg(unix)]
    pub(crate) fn fork<T: serde::Serialize + serde::de::DeserializeOwned>(
        f: impl FnOnce() -> T,
    ) -> nix::Result<T> {
        let (read_fd, write_fd) = nix::unistd::pipe()?;
        match unsafe { nix::unistd::fork() }? {
            nix::unistd::ForkResult::Parent { .. } => {
                let mut buff = [0u8; 8092];
                let len = nix::unistd::read(read_fd, &mut buff)?;
                if &buff[..len] == b"panic" {
                    ::core::panicking::panic_fmt(
                        format_args!("Benchmark function panicked"),
                    );
                }
                let t = serde_json::from_slice(&buff[..len]).unwrap();
                nix::unistd::close(read_fd)?;
                nix::unistd::close(write_fd)?;
                Ok(t)
            }
            nix::unistd::ForkResult::Child => {
                std::panic::set_hook({
                    let default_hook = std::panic::take_hook();
                    Box::new(move |panic_info| {
                        nix::unistd::write(write_fd, b"panic").unwrap();
                        default_hook(panic_info);
                    })
                });
                let t = f();
                let t_json = serde_json::to_string(&t).unwrap();
                nix::unistd::write(write_fd, t_json.as_bytes())?;
                std::process::exit(0);
            }
        }
    }
}
mod memory {
    use memory_stats::memory_stats;
    use std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        time::Duration,
    };
    /// Fork is only enabled on cfg(unix).
    /// If we ran this without fork, we would get incorrect results.
    ///
    /// When used with fork, the memory is immediately freed after the fork,
    /// so it doesn't affect the next benchmark run.
    #[cfg(unix)]
    pub(crate) fn monitor() -> impl FnOnce() -> Option<usize> {
        let stop_signal = Arc::new(AtomicBool::new(false));
        let handle = std::thread::spawn({
            let stop_signal = Arc::clone(&stop_signal);
            move || {
                let start_memory = memory_stats()?.physical_mem;
                let mut max_memory = 0;
                while !stop_signal.load(Ordering::Relaxed) {
                    let memory = memory_stats()?.physical_mem;
                    if memory > max_memory {
                        max_memory = memory;
                    }
                    std::thread::sleep(Duration::from_millis(1));
                }
                Some((start_memory, max_memory))
            }
        });
        move || {
            stop_signal.store(true, Ordering::Relaxed);
            let (start_memory, max_memory) = handle.join().unwrap()?;
            Some(max_memory - start_memory)
        }
    }
}
use fork::fork;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::time::{Duration, Instant};
#[allow(non_upper_case_globals)]
const _: () = {
    extern crate bench as _bench;
    fn wrapper(b: &mut bench::Bencher) {
        add(b);
    }
    _bench::register_bench(add, 1, wrapper);
};
fn add(b: &mut BenchmarkRun, p: u32) {}
pub struct Benchmark<'a> {
    name: &'a str,
    results: Vec<BenchmarkGroup>,
    #[serde(skip)]
    config: BenchmarkConfig,
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for Benchmark<'a> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Benchmark",
            "name",
            &self.name,
            "results",
            &self.results,
            "config",
            &&self.config,
        )
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'a> _serde::Serialize for Benchmark<'a> {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "Benchmark",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "name",
                &self.name,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "results",
                &self.results,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
impl<'a> Benchmark<'a> {
    pub fn new(name: &'a str) -> Self {
        add();
        Benchmark {
            name,
            config: BenchmarkConfig::default(),
            results: Vec::new(),
        }
    }
    pub fn with_config(name: &'a str, config: BenchmarkConfig) -> Self {
        Benchmark {
            name,
            config,
            results: Vec::new(),
        }
    }
    pub fn from_env(name: &'a str) -> Self {
        Benchmark {
            name,
            config: BenchmarkConfig::from_env(),
            results: Vec::new(),
        }
    }
    pub fn group(&mut self, name: &str) -> &mut BenchmarkGroup {
        let group = BenchmarkGroup::new(name.to_string(), &self.config);
        self.results.push(group);
        self.results.last_mut().unwrap()
    }
    pub fn benchmark<F: Fn(&mut BenchmarkRun)>(&mut self, name: &str, func: F) {
        let group = self.group(name);
        group.benchmark(name, func);
    }
    pub fn benchmark_with<F: Fn(&mut BenchmarkRun, &P) -> T, T, P: Debug>(
        &mut self,
        name: &str,
        params: &[(&str, P)],
        func: F,
    ) {
        let group = self.group(name);
        group.benchmark_with(params, func);
    }
    pub fn output(&self) {
        let output = ::serde_json::to_value(&self).unwrap();
        let output_str = serde_json::to_string_pretty(&output)
            .expect("failed to serialize");
        if let Some(path) = &self.config.output_dir {
            let path = std::path::Path::new(path);
            std::fs::create_dir_all(path).expect("failed to create output dir");
            std::fs::write(path.join(self.name).with_extension("json"), &output_str)
                .expect("failed to write output");
        }
        {
            ::std::io::_print(format_args!("{0}\n", &output_str));
        };
    }
}
pub struct BenchmarkConfig {
    pub quick: bool,
    pub output_dir: Option<String>,
}
#[automatically_derived]
impl ::core::fmt::Debug for BenchmarkConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "BenchmarkConfig",
            "quick",
            &self.quick,
            "output_dir",
            &&self.output_dir,
        )
    }
}
#[automatically_derived]
impl ::core::default::Default for BenchmarkConfig {
    #[inline]
    fn default() -> BenchmarkConfig {
        BenchmarkConfig {
            quick: ::core::default::Default::default(),
            output_dir: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for BenchmarkConfig {
    #[inline]
    fn clone(&self) -> BenchmarkConfig {
        BenchmarkConfig {
            quick: ::core::clone::Clone::clone(&self.quick),
            output_dir: ::core::clone::Clone::clone(&self.output_dir),
        }
    }
}
impl BenchmarkConfig {
    pub fn from_env() -> Self {
        let quick = env::var("BENCH_QUICK").unwrap_or("false".to_string());
        BenchmarkConfig {
            quick: quick == "true" || quick == "1",
            output_dir: env::var("BENCH_OUTPUT_DIR").ok(),
        }
    }
}
pub struct BenchmarkGroup {
    name: String,
    results: Vec<BenchmarkResult>,
    #[serde(skip)]
    config: BenchmarkConfig,
}
#[automatically_derived]
impl ::core::fmt::Debug for BenchmarkGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "BenchmarkGroup",
            "name",
            &self.name,
            "results",
            &self.results,
            "config",
            &&self.config,
        )
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for BenchmarkGroup {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "BenchmarkGroup",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "name",
                &self.name,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "results",
                &self.results,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BenchmarkGroup {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "name" => _serde::__private::Ok(__Field::__field0),
                        "results" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"name" => _serde::__private::Ok(__Field::__field0),
                        b"results" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<BenchmarkGroup>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = BenchmarkGroup;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct BenchmarkGroup",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct BenchmarkGroup with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        Vec<BenchmarkResult>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct BenchmarkGroup with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = _serde::__private::Default::default();
                    _serde::__private::Ok(BenchmarkGroup {
                        name: __field0,
                        results: __field1,
                        config: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<Vec<BenchmarkResult>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "results",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Vec<BenchmarkResult>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("name")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("results")?
                        }
                    };
                    _serde::__private::Ok(BenchmarkGroup {
                        name: __field0,
                        results: __field1,
                        config: _serde::__private::Default::default(),
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["name", "results"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "BenchmarkGroup",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<BenchmarkGroup>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl BenchmarkGroup {
    pub fn new(name: String, config: &BenchmarkConfig) -> Self {
        BenchmarkGroup {
            name,
            results: Vec::new(),
            config: config.clone(),
        }
    }
    pub fn group(&mut self, name: &str) -> &mut BenchmarkGroup {
        let group = BenchmarkGroup::new(name.to_string(), &self.config);
        self.results.push(BenchmarkResult::Group(group));
        match self.results.last_mut().unwrap() {
            BenchmarkResult::Group(ref mut group) => group,
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
    pub fn benchmark<F: Fn(&mut BenchmarkRun)>(&mut self, name: &str, func: F) {
        let run = fork(|| {
                let stop_monitoring_memory = memory::monitor();
                let mut run = BenchmarkRun::new(name.to_owned());
                func(&mut run);
                if let Some(memory_usage_bytes) = stop_monitoring_memory() {
                    run.log("memory_usage_bytes", memory_usage_bytes);
                }
                run
            })
            .unwrap();
        self.results.push(BenchmarkResult::Run(run));
    }
    pub fn benchmark_with<F: Fn(&mut BenchmarkRun, &P) -> T, T, P: Debug>(
        &mut self,
        params: &[(&str, P)],
        func: F,
    ) {
        let quick = self.config.quick;
        for p in params.iter().take(if quick { 1 } else { usize::MAX }) {
            let run = fork(|| {
                    let mut run = BenchmarkRun::new(p.0.to_owned());
                    func(&mut run, &p.1);
                    run
                })
                .unwrap();
            self.results.push(BenchmarkResult::Run(run));
        }
    }
}
#[serde(untagged)]
pub enum BenchmarkResult {
    Group(BenchmarkGroup),
    Run(BenchmarkRun),
}
#[automatically_derived]
impl ::core::fmt::Debug for BenchmarkResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            BenchmarkResult::Group(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Group", &__self_0)
            }
            BenchmarkResult::Run(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Run", &__self_0)
            }
        }
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for BenchmarkResult {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                BenchmarkResult::Group(ref __field0) => {
                    _serde::Serialize::serialize(__field0, __serializer)
                }
                BenchmarkResult::Run(ref __field0) => {
                    _serde::Serialize::serialize(__field0, __serializer)
                }
            }
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BenchmarkResult {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            let __content = <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                __deserializer,
            )?;
            let __deserializer = _serde::__private::de::ContentRefDeserializer::<
                __D::Error,
            >::new(&__content);
            if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                <BenchmarkGroup as _serde::Deserialize>::deserialize(__deserializer),
                BenchmarkResult::Group,
            ) {
                return _serde::__private::Ok(__ok);
            }
            if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                <BenchmarkRun as _serde::Deserialize>::deserialize(__deserializer),
                BenchmarkResult::Run,
            ) {
                return _serde::__private::Ok(__ok);
            }
            _serde::__private::Err(
                _serde::de::Error::custom(
                    "data did not match any variant of untagged enum BenchmarkResult",
                ),
            )
        }
    }
};
pub struct BenchmarkRun {
    pub name: String,
    pub time: Duration,
    pub metrics: HashMap<String, usize>,
}
#[automatically_derived]
impl ::core::fmt::Debug for BenchmarkRun {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "BenchmarkRun",
            "name",
            &self.name,
            "time",
            &self.time,
            "metrics",
            &&self.metrics,
        )
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for BenchmarkRun {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "BenchmarkRun",
                false as usize + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "name",
                &self.name,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "time",
                &self.time,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "metrics",
                &self.metrics,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BenchmarkRun {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "name" => _serde::__private::Ok(__Field::__field0),
                        "time" => _serde::__private::Ok(__Field::__field1),
                        "metrics" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"name" => _serde::__private::Ok(__Field::__field0),
                        b"time" => _serde::__private::Ok(__Field::__field1),
                        b"metrics" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<BenchmarkRun>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = BenchmarkRun;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct BenchmarkRun",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct BenchmarkRun with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        Duration,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct BenchmarkRun with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        HashMap<String, usize>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct BenchmarkRun with 3 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(BenchmarkRun {
                        name: __field0,
                        time: __field1,
                        metrics: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<Duration> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<
                        HashMap<String, usize>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("time"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<Duration>(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "metrics",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        HashMap<String, usize>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("name")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("time")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("metrics")?
                        }
                    };
                    _serde::__private::Ok(BenchmarkRun {
                        name: __field0,
                        time: __field1,
                        metrics: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["name", "time", "metrics"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "BenchmarkRun",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<BenchmarkRun>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl BenchmarkRun {
    fn new(name: String) -> Self {
        BenchmarkRun {
            name,
            time: Duration::new(0, 0),
            metrics: HashMap::new(),
        }
    }
    pub fn run<F, R>(&mut self, func: F) -> R
    where
        F: FnOnce() -> R,
    {
        let stop_monitoring_memory = memory::monitor();
        let start_time = Instant::now();
        let out = func();
        let elapsed_time = start_time.elapsed();
        self.time = elapsed_time;
        if let Some(memory_usage_bytes) = stop_monitoring_memory() {
            self.log("memory_usage_bytes", memory_usage_bytes);
        }
        out
    }
    pub fn log(&mut self, metric: &str, value: usize) {
        self.metrics.insert(metric.to_owned(), value);
    }
}
